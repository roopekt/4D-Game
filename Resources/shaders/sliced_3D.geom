#version 330 core
layout (triangles) in;
layout (triangle_strip, max_vertices = 4) out;// a quad

in GS_IN {
    vec3 world_position;
    vec3 world_normal;
} v_in[];

out FRAG_IN {
    vec3 world_position;
    vec3 world_normal;
} v_out;

struct Vertex {
    vec4 pos;
    vec3 world_pos;
    vec3 world_normal;
};

struct Line {
    Vertex A;
    Vertex B;
};

//statisfies mix(a, b, inverse_lerp_float(x, a, b)) == x
float inverse_lerp_float(float a, float b, float lerp) {
    return (lerp - a) / (b - a);
}

Vertex lerp_vertex(Vertex a, Vertex b, float t) {
    return Vertex(
        mix(a.pos, b.pos, t),
        mix(a.world_pos, b.world_pos, t),
        mix(a.world_normal, b.world_normal, t)
    );
}

Vertex get_intersection_point_for_line(Line line, out bool is_proper) {
    float t = inverse_lerp_float(line.A.pos.y, line.B.pos.y, 0.0);//because we want to output a vector with y == 0
    is_proper = (0.0 < t) && (t < 1.0);

    return lerp_vertex(line.A, line.B, t);
}

Vertex[3] get_intersections_for_triangle(Vertex[3] vertices, out int intersection_count) {
    Line[3] lines;
    lines[0] = Line(vertices[0], vertices[1]);
    lines[1] = Line(vertices[1], vertices[2]);
    lines[2] = Line(vertices[2], vertices[0]);

    intersection_count = 0;
    Vertex[3] intersections;
    for (int i = 0; i < 3; i++) {
        bool is_proper_intersection;
        Vertex new_intersection = get_intersection_point_for_line(lines[i], is_proper_intersection);
        if (is_proper_intersection) {
            intersections[intersection_count] = new_intersection;
            intersection_count++;
        }
    }

    return intersections;
}

Vertex[3] get_vertices() {
    Vertex[3] vertices;
    for (int i = 0; i < 3; i++) {
        vertices[i] = Vertex(
            gl_in[i].gl_Position,
            v_in[i].world_position,
            v_in[i].world_normal
        );
    }

    return vertices;
}

void emit_quad(Line line) {
    gl_Position = vec4(
        line.A.pos.x,
       -line.A.pos.w,
        line.A.pos.z,
        line.A.pos.w
    );
    v_out.world_position = line.A.world_pos;
    v_out.world_normal = line.A.world_normal;
    EmitVertex();

    gl_Position = vec4(
        line.A.pos.x,
        line.A.pos.w,
        line.A.pos.z,
        line.A.pos.w
    );
    v_out.world_position = line.A.world_pos;
    v_out.world_normal = line.A.world_normal;
    EmitVertex();

    gl_Position = vec4(
        line.B.pos.x,
       -line.B.pos.w,
        line.B.pos.z,
        line.B.pos.w
    );
    v_out.world_position = line.B.world_pos;
    v_out.world_normal = line.B.world_normal;
    EmitVertex();

    gl_Position = vec4(
        line.B.pos.x,
        line.B.pos.w,
        line.B.pos.z,
        line.B.pos.w
    );
    v_out.world_position = line.B.world_pos;
    v_out.world_normal = line.B.world_normal;
    EmitVertex();

    EndPrimitive();
}

//for debugging
void EmitUnchanged() {
    gl_Position = gl_in[0].gl_Position;
    v_out.world_position = v_in[0].world_position;
    v_out.world_normal = v_in[0].world_normal;
    EmitVertex();

    gl_Position = gl_in[1].gl_Position;
    v_out.world_position = v_in[1].world_position;
    v_out.world_normal = v_in[1].world_normal;
    EmitVertex();

    gl_Position = gl_in[2].gl_Position;
    v_out.world_position = v_in[2].world_position;
    v_out.world_normal = v_in[2].world_normal;
    EmitVertex();

    EndPrimitive();
}

void main() {
    Vertex[3] vertices = get_vertices();
    int intersection_count;
    Vertex[3] intersections = get_intersections_for_triangle(vertices, intersection_count);

    if (intersection_count != 2) {
        return;
    }

    emit_quad(Line(intersections[0], intersections[1]));
}
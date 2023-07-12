#version 330 core
layout (triangles) in;
layout (triangle_strip, max_vertices = 4) out;// a quad

in GS_IN {
    vec3 clip_position;
    float depth;
    vec3 world_position;
    vec3 world_normal;
} v_in[];

out FRAG_IN {
    vec3 world_position;
    vec3 world_normal;
    vec3 clip_position;
    float depth;
} v_out;

struct Vertex {
    vec3 clip_pos;
    float depth;
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
        mix(a.clip_pos, b.clip_pos, t),
        mix(a.depth, b.depth, t),
        mix(a.world_pos, b.world_pos, t),
        mix(a.world_normal, b.world_normal, t)
    );
}

Vertex get_intersection_point_for_line(Line line, out bool is_proper) {
    float t = inverse_lerp_float(line.A.clip_pos.x, line.B.clip_pos.x, 0.0);//because we want to output a vector with x == 0
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
            v_in[i].clip_position,
            v_in[i].depth,
            v_in[i].world_position,
            v_in[i].world_normal
        );
    }

    return vertices;
}

void emit_quad(Line line) {
    //note that perspective division happens after the geometry shader has run
    gl_Position = vec4(
       -line.A.depth,
        line.A.clip_pos.y,
        line.A.clip_pos.z,
        line.A.depth
    );
    v_out.world_position = line.A.world_pos;
    v_out.world_normal = line.A.world_normal;
    v_out.clip_position = line.A.clip_pos;
    v_out.depth = line.A.depth;
    EmitVertex();

    gl_Position = vec4(
        line.A.depth,
        line.A.clip_pos.y,
        line.A.clip_pos.z,
        line.A.depth
    );
    v_out.world_position = line.A.world_pos;
    v_out.world_normal = line.A.world_normal;
    v_out.clip_position = line.A.clip_pos;
    v_out.depth = line.A.depth;
    EmitVertex();

    gl_Position = vec4(
       -line.B.depth,
        line.B.clip_pos.y,
        line.B.clip_pos.z,
        line.B.depth
    );
    v_out.world_position = line.B.world_pos;
    v_out.world_normal = line.B.world_normal;
    v_out.clip_position = line.B.clip_pos;
    v_out.depth = line.B.depth;
    EmitVertex();

    gl_Position = vec4(
        line.B.depth,
        line.B.clip_pos.y,
        line.B.clip_pos.z,
        line.B.depth
    );
    v_out.world_position = line.B.world_pos;
    v_out.world_normal = line.B.world_normal;
    v_out.clip_position = line.B.clip_pos;
    v_out.depth = line.B.depth;
    EmitVertex();

    EndPrimitive();
}

//for debugging
void EmitUnchanged() {
    gl_Position = vec4(v_in[0].clip_position, v_in[0].depth);
    v_out.world_position = v_in[0].world_position;
    v_out.world_normal = v_in[0].world_normal;
    v_out.clip_position = v_in[0].clip_position;
    v_out.depth = v_in[0].depth;
    EmitVertex();

    gl_Position = vec4(v_in[1].clip_position, v_in[1].depth);
    v_out.world_position = v_in[1].world_position;
    v_out.world_normal = v_in[1].world_normal;
    v_out.clip_position = v_in[0].clip_position;
    v_out.depth = v_in[0].depth;
    EmitVertex();

    gl_Position = vec4(v_in[2].clip_position, v_in[2].depth);
    v_out.world_position = v_in[2].world_position;
    v_out.world_normal = v_in[2].world_normal;
    v_out.clip_position = v_in[0].clip_position;
    v_out.depth = v_in[0].depth;
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
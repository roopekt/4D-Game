#version 330 core
layout (lines_adjacency) in;//lines_adjacency simply means 4 vertices (a.k.a. a tetrahedron)
layout (triangle_strip, max_vertices = 4) out;

in GS_IN {
    vec4 clip_position;
    float depth;
    vec4 world_position;
    vec4 world_normal;
} v_in[];

out FRAG_IN {
    vec4 world_position;
    vec4 world_normal;
    vec4 clip_position;
    float depth;
} v_out;

struct Vertex {
    vec4 clip_pos;
    float depth;
    vec4 world_pos;
    vec4 world_normal;
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
    float t = inverse_lerp_float(line.A.clip_pos.x, line.B.clip_pos.x, 0.0);
    is_proper = (0.0 < t) && (t < 1.0);

    return lerp_vertex(line.A, line.B, t);
}

Vertex[6] get_intersections_for_tetrahedron(Vertex[4] vertices, out int intersection_count) {
    Line[6] lines;
    lines[0] = Line(vertices[0], vertices[1]);
    lines[1] = Line(vertices[0], vertices[2]);
    lines[2] = Line(vertices[0], vertices[3]);
    lines[3] = Line(vertices[1], vertices[2]);
    lines[4] = Line(vertices[1], vertices[3]);
    lines[5] = Line(vertices[2], vertices[3]);

    intersection_count = 0;
    Vertex[6] intersections;
    for (int i = 0; i < 6; i++) {
        bool is_proper_intersection;
        Vertex new_intersection = get_intersection_point_for_line(lines[i], is_proper_intersection);
        if (is_proper_intersection) {
            intersections[intersection_count] = new_intersection;
            intersection_count++;
        }
    }

    return intersections;
}

Vertex[4] get_vertices() {
    Vertex[4] vertices;
    for (int i = 0; i < 4; i++) {
        vertices[i] = Vertex(
            v_in[i].clip_position,
            v_in[i].depth,
            v_in[i].world_position,
            v_in[i].world_normal
        );
    }

    return vertices;
}

void emit_triangles(Vertex[6] intersections, int intersection_count) {
    bool is_triangle = intersection_count == 3;
    bool is_quadrilateral = intersection_count == 4;
    if (!(is_triangle || is_quadrilateral)) {
        return;
    }

    //note that perspective division happens after the geometry shader has run
    gl_Position = vec4(intersections[0].clip_pos.yzw, intersections[0].depth);
    v_out.world_position = intersections[0].world_pos;
    v_out.world_normal = intersections[0].world_normal;
    v_out.clip_position = intersections[0].clip_pos;
    v_out.depth = intersections[0].depth;
    EmitVertex();

    gl_Position = vec4(intersections[1].clip_pos.yzw, intersections[1].depth);
    v_out.world_position = intersections[1].world_pos;
    v_out.world_normal = intersections[1].world_normal;
    v_out.clip_position = intersections[1].clip_pos;
    v_out.depth = intersections[1].depth;
    EmitVertex();

    gl_Position = vec4(intersections[2].clip_pos.yzw, intersections[2].depth);
    v_out.world_position = intersections[2].world_pos;
    v_out.world_normal = intersections[2].world_normal;
    v_out.clip_position = intersections[2].clip_pos;
    v_out.depth = intersections[2].depth;
    EmitVertex();

    if (is_quadrilateral) {
        /* There are two ways to split a quadrilateral into triangles (which two vertices
        to join by a diagonal?), but the code gives out an arbitary triangulation. This is
        fine however, because both are valid, because in this case the quadrilateral is
        always convex.
    
        Proof:
          A shape is convex if and only if for any two points in the shape, the line segment
          connecting the points is fully enclosed by the shape. For our quadrilateral to be
          non-convex, there would have to be a pair of points for which the connecting
          line segment isn't enclosed by the quadrilateral. Our quadrilateral is a planar
          slice of a tetrahedron. Because the slice is planar, all points of the tetrahedron
          that are part of the connecting line are also part of the slice plane and thus the
          quadrilateral. Therefore, the whole tetrahedron couldn't enclose the line either,
          meaning that the tetrahedron is also non-convex. All tetrahedra are convex, so this
          is a contradiction, and the quadrilateral must be convex.
    
        The connecting-line-property also shows why both triangulations of the quadrilateral
        are correct. */

        gl_Position = vec4(intersections[3].clip_pos.yzw, intersections[3].depth);
        v_out.world_position = intersections[3].world_pos;
        v_out.world_normal = intersections[3].world_normal;
        v_out.clip_position = intersections[3].clip_pos;
        v_out.depth = intersections[3].depth;
        EmitVertex();
    }

    EndPrimitive();
}

void main() {
    Vertex[4] vertices = get_vertices();

    int intersection_count;
    Vertex[6] intersections = get_intersections_for_tetrahedron(vertices, intersection_count);

    emit_triangles(intersections, intersection_count);
}
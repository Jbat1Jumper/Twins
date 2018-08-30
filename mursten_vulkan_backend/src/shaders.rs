pub mod vs {
    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[src = "
        #version 450
        
        const float PI = 3.1415926535897932384626433832795;
        const float PI_2 = 1.57079632679489661923;
        const float PI_4 = 0.785398163397448309616;

        layout(location = 0) in vec4 position;
        layout(location = 4) in vec4 normal;
        layout(location = 8) in vec4 color;
        layout(location = 12) in vec2 texture;
        layout(location = 0) out vec4 outColor;
        layout(location = 4) out vec4 outNormal;

        layout(push_constant) uniform pushConstants {
            mat4 world;
            mat4 view;
            mat4 projection;

            vec4 ambient_color;
            vec4 diffuse_color;
            vec4 diffuse_direction;

            float scale;
            float ambient_strength;
            float diffuse_strength;
        } c;

        void main() {
            mat4 scale = mat4(
                c.scale, 0, 0, 0,
                0, c.scale, 0, 0,
                0, 0, c.scale, 0,
                0, 0, 0, 1
            );
            gl_Position = c.projection * c.view * c.world * scale * position;
            outColor = color;
            outNormal = normal;
        }
    "]
    struct Dummy;
}

pub mod fs {
    #[derive(VulkanoShader)]
    #[ty = "fragment"]
    #[src = "
        #version 450

        layout(location = 0) in vec4 inColor;
        layout(location = 4) in vec4 inNormal;
        layout(location = 0) out vec4 outColor;

        float rand(vec2 co) {
            return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
        }

        void main() {
            outColor = inColor;
        }
    "]
    struct Dummy;
}

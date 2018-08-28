pub mod vs {
    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[src = "
        #version 450
        
        const float PI = 3.1415926535897932384626433832795;
        const float PI_2 = 1.57079632679489661923;
        const float PI_4 = 0.785398163397448309616;

        layout(location = 0) in vec4 position;
        layout(location = 4) in vec4 color;
        layout(location = 8) in vec2 texture;
        layout(location = 0) out vec4 outColor;

        layout(push_constant) uniform pushConstants {
            mat4 world;
            mat4 view;
            mat4 projection;
            float scale;
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

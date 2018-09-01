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
        layout(location = 4) out vec4 outFragPos;
        layout(location = 8) out vec4 outNormal;

        layout(push_constant) uniform pushConstants {
            mat4 world;
            mat4 view;
            mat4 projection;

            vec4 ambient_color;
            vec4 diffuse_color;
            vec4 diffuse_origin;
            vec4 specular_color;

            float scale;
            float ambient_strength;
            float diffuse_strength;
            float specular_strength;
        } c;

        void main() {
            gl_Position = c.projection * c.view * c.world * position;
            gl_Position.y = -gl_Position.y;
            gl_Position.z = (gl_Position.z + gl_Position.w) / 2.0;

            outColor = color;

            outFragPos = position;

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
        layout(location = 4) in vec4 inFragPos;
        layout(location = 8) in vec4 inNormal;
        layout(location = 0) out vec4 outColor;

        layout(push_constant) uniform pushConstants {
            mat4 world;
            mat4 view;
            mat4 projection;

            vec4 ambient_color;
            vec4 diffuse_color;
            vec4 diffuse_origin;
            vec4 specular_color;

            float scale;
            float ambient_strength;
            float diffuse_strength;
            float specular_strength;
        } c;

        float rand(vec2 co) {
            return fract(sin(dot(co.xy, vec2(12.9898,78.233))) * 43758.5453);
        }

        void main() {
            vec4 ambient = c.ambient_strength * c.ambient_color;
            ambient.w = 1.0;

            vec4 norm = normalize(inNormal);
            vec4 diffuse_origin = c.diffuse_origin;
            vec4 lightDir = normalize(diffuse_origin - inFragPos);  

            float diff = max(dot(norm, lightDir), 0.0);
            vec4 diffuse = diff * c.diffuse_color;


            vec4 viewPos = c.view * vec4(0, 0, 0, 1);
            vec4 viewDir = normalize(viewPos - inFragPos);
            vec4 reflectDir = reflect(-lightDir, norm); 

            float spec = pow(max(dot(viewDir, reflectDir), 0.0), 128);
            vec4 specular = c.specular_strength * spec * c.specular_color;  
            specular.w = 1.0;

            outColor = inColor * (ambient + diffuse + specular);
        }
    "]
    struct Dummy;
}

use crate::crowengine::shaders::shader::Shader;
use crate::crowengine::world::World;

pub struct UseShader {
    pub shader_name: String,
}

impl UseShader {
    pub fn new(shader_name: String) -> Self {
        Self { shader_name }
    }

    pub fn get_shader<'a>(&self, world: &'a World) -> &'a Shader {
        world
            .resources
            .shaders
            .get(self.shader_name.as_str())
            .unwrap_or_else(|| {
                panic!("Shader '{}' not found in resources!", self.shader_name);
            })
    }

    pub fn get_optional_shader<'a>(use_shader: Option<&Self>, world: &'a World) -> &'a Shader {
        let shader_name = use_shader
            .map(|s| s.shader_name.as_str())
            .unwrap_or("default");

        world.resources.shaders.get(shader_name).unwrap_or_else(|| {
            panic!("Shader '{}' not found in resources!", shader_name);
        })
    }
}

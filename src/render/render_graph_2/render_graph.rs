use crate::render::render_graph_2::{
    PassDescriptor, PipelineDescriptor, ResourceProvider, TextureDescriptor,
};
use std::collections::HashMap;

pub struct RenderGraph {
    pub pipeline_descriptors: HashMap<String, PipelineDescriptor>,
    // TODO: make this ordered
    pub pass_descriptors: HashMap<String, PassDescriptor>,
    pub pass_pipelines: HashMap<String, Vec<String>>,
    pub resource_providers: Vec<Box<dyn ResourceProvider>>,
    pub queued_textures: Vec<(String, TextureDescriptor)>,
}

impl Default for RenderGraph {
    fn default() -> Self {
        RenderGraph {
            pipeline_descriptors: HashMap::new(),
            pass_descriptors: HashMap::new(),
            pass_pipelines: HashMap::new(),
            resource_providers: Vec::new(),
            queued_textures: Vec::new(),
        }
    }
}

pub struct RenderGraphBuilder {
    render_graph: RenderGraph,
    current_pass: Option<String>,
}

impl RenderGraphBuilder {
    pub fn new() -> Self {
        RenderGraphBuilder {
            render_graph: RenderGraph::default(),
            current_pass: None,
        }
    }

    pub fn add_pass(mut self, name: &str, pass: PassDescriptor) -> Self {
        self.current_pass = Some(name.to_string());
        self.render_graph
            .pass_descriptors
            .insert(name.to_string(), pass);
        self
    }

    pub fn add_pipeline(mut self, name: &str, pipeline: PipelineDescriptor) -> Self {
        self.render_graph
            .pipeline_descriptors
            .insert(name.to_string(), pipeline);

        if let Some(current_pass) = self.current_pass.as_ref() {
            if let None = self.render_graph.pass_pipelines.get(current_pass) {
                self.render_graph
                    .pass_pipelines
                    .insert(current_pass.to_string(), Vec::new());
            }

            let pass_pipelines = self
                .render_graph
                .pass_pipelines
                .get_mut(current_pass)
                .unwrap();
            pass_pipelines.push(name.to_string());
        }

        self
    }

    pub fn add_resource_provider(mut self, resource_provider: Box<dyn ResourceProvider>) -> Self {
        self.render_graph.resource_providers.push(resource_provider);
        self
    }

    pub fn add_texture(mut self, name: &str, texture_descriptor: TextureDescriptor) -> Self {
        self.render_graph
            .queued_textures
            .push((name.to_string(), texture_descriptor));
        self
    }

    pub fn build(self) -> RenderGraph {
        self.render_graph
    }
}
pub trait ReasoningContentRemappingTarget {
    type Inner: ReasoningContentRemappingInnerTarget;

    fn index(&self) -> usize;
    fn inner_mut_opt(&mut self) -> Option<&mut Self::Inner>;
}

pub trait ReasoningContentRemappingInnerTarget {
    fn content_mut(&mut self) -> &mut Option<String>;
    fn reasoning_content_mut(&mut self) -> &mut Option<String>;
    fn reasoning_mut(&mut self) -> &mut Option<String>;
}

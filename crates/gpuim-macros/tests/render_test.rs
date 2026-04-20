#[test]
fn test_derive_render() {
    use gpuim_macros::Render;

    #[derive(Render)]
    struct _Element;
}

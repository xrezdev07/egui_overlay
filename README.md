## Egui Overlay
In this context, Overlay simply means a gui application where
1. It has a transparent window surface
2. It can toggle the borders/decorations like title bar.
3. **can** let the input like mouse clicks to passthrough its window surface.

Here, we will let input passthrough when egui doesn't need input. 

The project itself builds upon the `egui_backend` crate while using `egui_window_glfw_passthrough` for windowing functionality. 
For rendering, we use `egui_render_three_d`, as `three-d` will allow you to draw a bunch of things easily. 

For advanced usecases, i recommend directly using `egui_window_glfw_passthrough` from https://github.com/coderedart/etk

Look at the `hello` example for a rough idea of how to use this crate.



https://github.com/coderedart/egui_overlay/assets/24411704/9f7bab7b-26ec-47d1-b51e-74006dfa7b0d

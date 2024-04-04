use leptos::{html::Div, *};
use leptos_use::{
    use_draggable_with_options, use_drop_zone_with_options, UseDraggableOptions, UseDropZoneOptions,
};
use log::info;

const OUTER_CLASS: &str =
    "flex flex-col gap-2 p-10 h-min w-min border-dashed border-black border-2";
const BOX_CLASS: &str = "h-8 w-8 border-dashed border-black border-2";

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (dragging_box, set_dragging_box) = create_signal("None");
    let (over_box, set_over_box) = create_signal(None);

    let (boxes, set_boxes) = create_signal(vec!["A", "B", "C", "D"]);

    let nodes = boxes()
        .iter()
        .enumerate()
        .map(|(index, r#box)| (index, *r#box, create_node_ref::<Div>()))
        .map(|(index, r#box, node)| {
            let _drop_options = use_drop_zone_with_options(
                node,
                UseDropZoneOptions::default()
                    .on_over(move |_| {
                        set_over_box(Some(r#box));
                    })
                    .on_leave(move |_| {
                        set_over_box(None);
                    })
                    .on_drop(move |_| {
                        set_over_box(None);
                        info!("Dropped {} onto {}", dragging_box(), r#box);

                        set_boxes.update(|boxes| {
                            let drag_index =
                                boxes.iter().position(|b| *b == dragging_box()).unwrap();
                            let drop_index = boxes.iter().position(|b| *b == r#box).unwrap();

                            boxes.swap(drag_index, drop_index);
                        });

                        info!("Boxes: {:?}", boxes());
                    }),
            );
            let _drag_options = use_draggable_with_options(
                node,
                UseDraggableOptions::default().on_start(move |_| {
                    set_dragging_box(r#box);
                    true
                }),
            );

            (index, node)
        })
        .collect::<Vec<(usize, NodeRef<Div>)>>();

    let enumerated_boxes: Memo<Vec<(usize, &str)>> =
        create_memo(move |_| boxes().into_iter().enumerate().collect());

    view! {
      <div>
        <ul class=OUTER_CLASS>
            <For
              each=enumerated_boxes
              key=|(_, r#box)| *r#box
              children=move |(index, r#box)|
              {
                let node: (usize, NodeRef<Div>) = nodes[index];
                let node_ref = node.1;

                let is_dragged_over = move || over_box().as_ref().map(|b| *b == r#box).unwrap_or(false);
                let is_over_class = move || if is_dragged_over() { "bg-gray-200" } else { "" };
                let class = move || format!("{} {}", BOX_CLASS, is_over_class());
                view!{
                <li>
                  <div key=index class=class draggable="true" node_ref=node_ref>
                    {r#box}
                  </div>
                </li>
              }}
            />
        </ul>
        <Show
          when=move || over_box().is_some()
          fallback=|| view! { <div>Not over zone</div> }
          >
          <div>Over zone {over_box().unwrap()}</div>
        </Show>
      </div>
    }
}

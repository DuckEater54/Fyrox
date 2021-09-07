use crate::{
    core::{algebra::Vector2, color::Color, pool::Handle},
    grid::{Column, GridBuilder, Row},
    message::{
        MessageData, MessageDirection, NumericUpDownMessage, UiMessage, UiMessageData,
        Vec2EditorMessage,
    },
    node::UINode,
    vec::{make_mark, make_numeric_input},
    BuildContext, Control, NodeHandleMapping, UserInterface, Widget, WidgetBuilder,
};
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct Vec2Editor<M: MessageData, C: Control<M, C>> {
    widget: Widget<M, C>,
    x_field: Handle<UINode<M, C>>,
    y_field: Handle<UINode<M, C>>,
    value: Vector2<f32>,
}

crate::define_widget_deref!(Vec2Editor<M, C>);

impl<M: MessageData, C: Control<M, C>> Control<M, C> for Vec2Editor<M, C> {
    fn resolve(&mut self, node_map: &NodeHandleMapping<M, C>) {
        node_map.resolve(&mut self.x_field);
        node_map.resolve(&mut self.y_field);
    }

    fn handle_routed_message(
        &mut self,
        ui: &mut UserInterface<M, C>,
        message: &mut UiMessage<M, C>,
    ) {
        self.widget.handle_routed_message(ui, message);

        match *message.data() {
            UiMessageData::NumericUpDown(NumericUpDownMessage::Value(value))
                if message.direction() == MessageDirection::FromWidget =>
            {
                if message.destination() == self.x_field {
                    ui.send_message(Vec2EditorMessage::value(
                        self.handle(),
                        MessageDirection::ToWidget,
                        Vector2::new(value, self.value.y),
                    ));
                } else if message.destination() == self.y_field {
                    ui.send_message(Vec2EditorMessage::value(
                        self.handle(),
                        MessageDirection::ToWidget,
                        Vector2::new(self.value.x, value),
                    ));
                }
            }
            UiMessageData::Vec2Editor(Vec2EditorMessage::Value(value))
                if message.direction() == MessageDirection::ToWidget =>
            {
                let mut changed = false;
                if self.value.x != value.x {
                    self.value.x = value.x;
                    ui.send_message(NumericUpDownMessage::value(
                        self.x_field,
                        MessageDirection::ToWidget,
                        value.x,
                    ));
                    changed = true;
                }
                if self.value.y != value.y {
                    self.value.y = value.y;
                    ui.send_message(NumericUpDownMessage::value(
                        self.y_field,
                        MessageDirection::ToWidget,
                        value.y,
                    ));
                    changed = true;
                }
                if changed {
                    ui.send_message(message.reverse());
                }
            }
            _ => (),
        }
    }
}

pub struct Vec2EditorBuilder<M: MessageData, C: Control<M, C>> {
    widget_builder: WidgetBuilder<M, C>,
    value: Vector2<f32>,
}

impl<M: MessageData, C: Control<M, C>> Vec2EditorBuilder<M, C> {
    pub fn new(widget_builder: WidgetBuilder<M, C>) -> Self {
        Self {
            widget_builder,
            value: Default::default(),
        }
    }

    pub fn with_value(mut self, value: Vector2<f32>) -> Self {
        self.value = value;
        self
    }

    pub fn build(self, ctx: &mut BuildContext<M, C>) -> Handle<UINode<M, C>> {
        let x_field;
        let y_field;
        let grid = GridBuilder::new(
            WidgetBuilder::new()
                .with_child(make_mark(ctx, "X", 0, Color::opaque(120, 0, 0)))
                .with_child({
                    x_field = make_numeric_input(ctx, 1);
                    x_field
                })
                .with_child(make_mark(ctx, "Y", 2, Color::opaque(0, 120, 0)))
                .with_child({
                    y_field = make_numeric_input(ctx, 3);
                    y_field
                }),
        )
        .add_row(Row::stretch())
        .add_column(Column::auto())
        .add_column(Column::stretch())
        .add_column(Column::auto())
        .add_column(Column::stretch())
        .build(ctx);

        let node = Vec2Editor {
            widget: self.widget_builder.with_child(grid).build(),
            x_field,
            y_field,
            value: self.value,
        };

        ctx.add_node(UINode::Vec2Editor(node))
    }
}
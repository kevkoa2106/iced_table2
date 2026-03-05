use iced_widget::container;

/// Controls the visual appearance of a [`Table`](crate::Table).
///
/// Implement this trait on your theme type to customize how headers, footers,
/// rows, and column dividers are rendered. A default implementation is provided
/// for [`iced_core::Theme`] with a strong background for the header/footer,
/// alternating row colors, and a highlight effect on hovered dividers.
pub trait Catalog {
    /// The style value that selects a particular appearance variant.
    ///
    /// For simple themes this can be `()`. For themes with multiple table
    /// styles (e.g. "compact" vs "striped"), use an enum.
    type Style: Default + Clone;

    /// Returns the [`container::Style`] for the header row.
    fn header(&self, style: &Self::Style) -> container::Style;

    /// Returns the [`container::Style`] for the footer row.
    fn footer(&self, style: &Self::Style) -> container::Style;

    /// Returns the [`container::Style`] for a body row at the given `index`.
    ///
    /// The index can be used to produce alternating row colors.
    fn row(&self, style: &Self::Style, index: usize) -> container::Style;

    /// Returns the [`container::Style`] for a selected body row at the given `index`.
    ///
    /// By default this falls back to [`row()`](Self::row). Override it to
    /// give selected rows a distinct appearance (e.g. a highlighted background).
    fn selected_row(&self, style: &Self::Style, index: usize) -> container::Style {
        self.row(style, index)
    }

    /// Returns the [`container::Style`] for a column divider.
    ///
    /// `hovered` is `true` when the cursor is over the divider or a drag is
    /// in progress, allowing you to provide visual feedback.
    fn divider(&self, style: &Self::Style, hovered: bool) -> container::Style;
}

impl Catalog for iced_core::Theme {
    type Style = ();

    fn header(&self, _style: &Self::Style) -> container::Style {
        container::Style {
            text_color: Some(self.extended_palette().background.strong.text),
            background: Some(self.extended_palette().background.strong.color.into()),
            ..Default::default()
        }
    }

    fn footer(&self, style: &Self::Style) -> container::Style {
        self.header(style)
    }

    fn row(&self, _style: &Self::Style, index: usize) -> container::Style {
        let pair = if index % 2 == 0 {
            self.extended_palette().background.base
        } else {
            self.extended_palette().background.weak
        };

        container::Style {
            text_color: Some(pair.text),
            background: Some(pair.color.into()),
            ..Default::default()
        }
    }

    fn selected_row(&self, _style: &Self::Style, _index: usize) -> container::Style {
        let pair = self.extended_palette().primary.weak;

        container::Style {
            text_color: Some(pair.text),
            background: Some(pair.color.into()),
            ..Default::default()
        }
    }

    fn divider(&self, _style: &Self::Style, hovered: bool) -> container::Style {
        let pair = if hovered {
            self.extended_palette().primary.base
        } else {
            self.extended_palette().background.weak
        };

        container::Style {
            background: Some(pair.color.into()),
            ..Default::default()
        }
    }
}

pub(crate) mod wrapper {
    use iced_core::{mouse::Cursor, Color, Element, Length, Size, Vector, Widget};
    use iced_widget::container;

    pub fn header<'a, Message, Theme, Renderer>(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        style: <Theme as super::Catalog>::Style,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: super::Catalog + 'a,
        Message: 'a,
    {
        Wrapper {
            content: content.into(),
            target: Target::Header,
            style,
        }
        .into()
    }

    pub fn footer<'a, Message, Theme, Renderer>(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        style: <Theme as super::Catalog>::Style,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: super::Catalog + 'a,
        Message: 'a,
    {
        Wrapper {
            content: content.into(),
            target: Target::Footer,
            style,
        }
        .into()
    }

    pub fn row<'a, Message, Theme, Renderer>(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        style: <Theme as super::Catalog>::Style,
        index: usize,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: super::Catalog + 'a,
        Message: 'a,
    {
        Wrapper {
            content: content.into(),
            target: Target::Row { index },
            style,
        }
        .into()
    }

    pub fn selected_row<'a, Message, Theme, Renderer>(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        style: <Theme as super::Catalog>::Style,
        index: usize,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: super::Catalog + 'a,
        Message: 'a,
    {
        Wrapper {
            content: content.into(),
            target: Target::SelectedRow { index },
            style,
        }
        .into()
    }

    enum Target {
        Header,
        Footer,
        Row { index: usize },
        SelectedRow { index: usize },
    }

    impl Target {
        fn appearance<Theme>(
            &self,
            theme: &Theme,
            style: &<Theme as super::Catalog>::Style,
        ) -> container::Style
        where
            Theme: super::Catalog,
        {
            match self {
                Target::Header => theme.header(style),
                Target::Footer => theme.footer(style),
                Target::Row { index } => theme.row(style, *index),
                Target::SelectedRow { index } => theme.selected_row(style, *index),
            }
        }
    }

    struct Wrapper<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer,
        Theme: super::Catalog,
    {
        content: Element<'a, Message, Theme, Renderer>,
        target: Target,
        style: <Theme as super::Catalog>::Style,
    }

    impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
        for Wrapper<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer,
        Theme: super::Catalog,
    {
        fn size(&self) -> Size<Length> {
            self.content.as_widget().size()
        }

        fn layout(
            &mut self,
            state: &mut iced_core::widget::Tree,
            renderer: &Renderer,
            limits: &iced_core::layout::Limits,
        ) -> iced_core::layout::Node {
            self.content
                .as_widget_mut()
                .layout(state, renderer, limits)
        }

        fn draw(
            &self,
            state: &iced_core::widget::Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &iced_core::renderer::Style,
            layout: iced_core::Layout<'_>,
            cursor: Cursor,
            viewport: &iced_core::Rectangle,
        ) {
            let appearance = self.target.appearance::<Theme>(theme, &self.style);

            renderer.fill_quad(
                iced_core::renderer::Quad {
                    bounds: layout.bounds(),
                    border: appearance.border,
                    shadow: Default::default(),
                    snap: true,
                },
                appearance
                    .background
                    .unwrap_or_else(|| Color::TRANSPARENT.into()),
            );

            let style = appearance
                .text_color
                .map(|text_color| iced_core::renderer::Style { text_color })
                .unwrap_or(*style);

            self.content
                .as_widget()
                .draw(state, renderer, theme, &style, layout, cursor, viewport)
        }

        fn tag(&self) -> iced_core::widget::tree::Tag {
            self.content.as_widget().tag()
        }

        fn state(&self) -> iced_core::widget::tree::State {
            self.content.as_widget().state()
        }

        fn children(&self) -> Vec<iced_core::widget::Tree> {
            self.content.as_widget().children()
        }

        fn diff(&self, tree: &mut iced_core::widget::Tree) {
            self.content.as_widget().diff(tree)
        }

        fn operate(
            &mut self,
            state: &mut iced_core::widget::Tree,
            layout: iced_core::Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn iced_core::widget::Operation,
        ) {
            self.content
                .as_widget_mut()
                .operate(state, layout, renderer, operation)
        }

        fn update(
            &mut self,
            state: &mut iced_core::widget::Tree,
            event: &iced_core::Event,
            layout: iced_core::Layout<'_>,
            cursor: Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn iced_core::Clipboard,
            shell: &mut iced_core::Shell<'_, Message>,
            viewport: &iced_core::Rectangle,
        ) {
            self.content.as_widget_mut().update(
                state, event, layout, cursor, renderer, clipboard, shell, viewport,
            );
        }

        fn mouse_interaction(
            &self,
            state: &iced_core::widget::Tree,
            layout: iced_core::Layout<'_>,
            cursor: Cursor,
            viewport: &iced_core::Rectangle,
            renderer: &Renderer,
        ) -> iced_core::mouse::Interaction {
            self.content
                .as_widget()
                .mouse_interaction(state, layout, cursor, viewport, renderer)
        }

        fn overlay<'b>(
            &'b mut self,
            state: &'b mut iced_core::widget::Tree,
            layout: iced_core::Layout<'b>,
            renderer: &Renderer,
            viewport: &iced_core::Rectangle,
            translation: Vector,
        ) -> Option<iced_core::overlay::Element<'b, Message, Theme, Renderer>> {
            self.content
                .as_widget_mut()
                .overlay(state, layout, renderer, viewport, translation)
        }
    }

    impl<'a, Message, Theme, Renderer> From<Wrapper<'a, Message, Theme, Renderer>>
        for Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: super::Catalog + 'a,
        Message: 'a,
    {
        fn from(wrapper: Wrapper<'a, Message, Theme, Renderer>) -> Self {
            Element::new(wrapper)
        }
    }
}

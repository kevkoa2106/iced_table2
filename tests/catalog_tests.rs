use iced_core::Theme;
use iced_table2::Catalog;

#[test]
fn header_style() {
    let theme = Theme::Light;
    let style = theme.header(&());

    assert!(style.background.is_some());
    assert!(style.text_color.is_some());
}

#[test]
fn footer_matches_header() {
    let theme = Theme::Light;
    let header = theme.header(&());
    let footer = theme.footer(&());

    assert_eq!(
        format!("{:?}", header.background),
        format!("{:?}", footer.background)
    );
    assert_eq!(
        format!("{:?}", header.text_color),
        format!("{:?}", footer.text_color)
    );
}

#[test]
fn alternating_row_colors() {
    let theme = Theme::Light;
    let even = theme.row(&(), 0);
    let odd = theme.row(&(), 1);

    assert!(even.background.is_some());
    assert!(odd.background.is_some());
    assert_ne!(
        format!("{:?}", even.background),
        format!("{:?}", odd.background)
    );
}

#[test]
fn same_parity_rows_match() {
    let theme = Theme::Light;
    let row0 = theme.row(&(), 0);
    let row2 = theme.row(&(), 2);
    let row1 = theme.row(&(), 1);
    let row3 = theme.row(&(), 3);

    assert_eq!(
        format!("{:?}", row0.background),
        format!("{:?}", row2.background)
    );
    assert_eq!(
        format!("{:?}", row1.background),
        format!("{:?}", row3.background)
    );
}

#[test]
fn selected_row_differs_from_normal() {
    let theme = Theme::Light;
    let normal = theme.row(&(), 0);
    let selected = theme.selected_row(&(), 0);

    assert_ne!(
        format!("{:?}", normal.background),
        format!("{:?}", selected.background)
    );
}

#[test]
fn divider_hovered_differs() {
    let theme = Theme::Light;
    let normal = theme.divider(&(), false);
    let hovered = theme.divider(&(), true);

    assert!(normal.background.is_some());
    assert!(hovered.background.is_some());
    assert_ne!(
        format!("{:?}", normal.background),
        format!("{:?}", hovered.background)
    );
}

#[test]
fn row_has_no_border() {
    let theme = Theme::Light;
    let row = theme.row(&(), 0);

    assert_eq!(row.border, Default::default());
}

#[test]
fn header_has_no_border() {
    let theme = Theme::Light;
    let header = theme.header(&());

    assert_eq!(header.border, Default::default());
}

#[test]
fn catalog_style_is_unit() {
    let _style: <Theme as Catalog>::Style = ();
}

#[test]
fn default_theme_works_with_all_builtin_themes() {
    let themes = [
        Theme::Light,
        Theme::Dark,
        Theme::Dracula,
        Theme::Nord,
        Theme::SolarizedLight,
        Theme::SolarizedDark,
        Theme::GruvboxLight,
        Theme::GruvboxDark,
        Theme::CatppuccinLatte,
        Theme::CatppuccinFrappe,
        Theme::CatppuccinMacchiato,
        Theme::CatppuccinMocha,
        Theme::TokyoNight,
        Theme::TokyoNightStorm,
        Theme::TokyoNightLight,
        Theme::KanagawaWave,
        Theme::KanagawaDragon,
        Theme::KanagawaLotus,
        Theme::Moonfly,
        Theme::Nightfly,
        Theme::Oxocarbon,
        Theme::Ferra,
    ];

    for theme in &themes {
        let _header = theme.header(&());
        let _footer = theme.footer(&());
        let _row_even = theme.row(&(), 0);
        let _row_odd = theme.row(&(), 1);
        let _selected = theme.selected_row(&(), 0);
        let _divider = theme.divider(&(), false);
        let _divider_hovered = theme.divider(&(), true);
    }
}

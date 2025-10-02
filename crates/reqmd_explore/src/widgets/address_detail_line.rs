use super::prelude::*;

#[derive(Debug, Clone)]
pub struct AddressDetailLine<'a> {
    address: &'a http::Address,
}

impl Widget for AddressDetailLine<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let line = Line::from_iter([
            Span::raw(" Address: ").black().on_gray(),
            Span::raw(" ").gray().on_black(),
            Span::raw(self.address.build_url().to_string())
                .gray()
                .on_black()
                .bold(),
        ]);

        line.render(area, buf);
    }
}

impl<'a> From<&'a http::Address> for AddressDetailLine<'a> {
    fn from(address: &'a http::Address) -> Self {
        Self { address }
    }
}

impl<'a> From<&'a http::Request> for AddressDetailLine<'a> {
    fn from(request: &'a http::Request) -> Self {
        Self::from(&request.address)
    }
}

use std::borrow::Cow;
use std::str::FromStr;
use std::num::ParseIntError;
use std::path::PathBuf;

use hyper::Uri;
use url::Url;

use super::error::ServiceError;

pub fn queried_path(uri: &Uri) -> Result<PathBuf, ServiceError> {
    let url = url(uri)?;

    query_parameter(&url, "path").map(|value| PathBuf::from(value.as_ref()))
}

pub fn queried_indices(uri: &Uri) -> Result<(usize, usize), ServiceError> {
    let url = url(uri)?;

    let start = integer_query_parameter(&url, "start")?;
    let end = integer_query_parameter(&url, "end")?;

    Ok((start, end))
}

pub fn queried_dimensions(uri: &Uri) -> Result<(u32, u32), ServiceError> {
    let url = url(uri)?;

    let max_width = integer_query_parameter(&url, "maxWidth")?;
    let max_height = integer_query_parameter(&url, "maxHeight")?;

    Ok((max_width, max_height))
}

fn url(uri: &Uri) -> Result<Url, ServiceError> {
    let base_url = Url::parse("http://127.0.0.1/")?;
    let parse_options = Url::options().base_url(Some(&base_url));
    parse_options.parse(uri.as_ref()).map_err(
        ServiceError::UrlParseError,
    )
}

fn integer_query_parameter<T: FromStr<Err = ParseIntError>>(
    url: &Url,
    parameter: &'static str,
) -> Result<T, ServiceError> {
    query_parameter(url, parameter).and_then(|value| {
        value.parse().map_err(
            ServiceError::QueryParameterParseError,
        )
    })
}

fn query_parameter<'a>(
    url: &'a Url,
    parameter: &'static str,
) -> Result<Cow<'a, str>, ServiceError> {
    url.query_pairs()
        .find(|pair| pair.0 == parameter)
        .map(|pair| pair.1)
        .ok_or(ServiceError::MissingQueryParameter(parameter))
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::Path;

    #[test]
    fn queried_path_should_extract_start_and_end_query_parameter_values() {
        let uri = Uri::from_str("http://127.0.0.1/?path=test").unwrap();
        let path = queried_path(&uri).unwrap();

        assert_eq!(Path::new("test"), path);
    }

    #[test]
    fn queried_path_should_error_if_the_path_parameter_does_not_exist() {
        let uri = Uri::from_str("http://127.0.0.1/").unwrap();
        let result = queried_path(&uri);

        assert!(match result {
            Err(ServiceError::MissingQueryParameter("path")) => true,
            _ => false,
        });
    }

    #[test]
    fn queried_indices_should_extract_start_and_end_query_parameter_values() {
        let uri = Uri::from_str("http://127.0.0.1/?start=1&end=2").unwrap();
        let (start, end) = queried_indices(&uri).unwrap();

        assert_eq!(1, start);
        assert_eq!(2, end);
    }

    #[test]
    fn queried_dimensions_should_extract_max_width_and_max_height_query_parameter_values() {
        let uri = Uri::from_str("http://127.0.0.1/?maxWidth=1&maxHeight=2").unwrap();
        let (width, height) = queried_dimensions(&uri).unwrap();

        assert_eq!(1, width);
        assert_eq!(2, height);
    }

    #[test]
    fn integer_query_parameter_should_extract_the_value_for_the_given_query_parameters() {
        let url = Url::parse("http://127.0.0.1/?test=1").unwrap();
        let result = integer_query_parameter::<u32>(&url, "test");

        assert_eq!(1, result.unwrap());
    }

    #[test]
    fn integer_query_parameter_should_error_if_the_parameter_does_not_exist() {
        let url = Url::parse("http://127.0.0.1/").unwrap();
        let result = integer_query_parameter::<u32>(&url, "test");

        assert!(match result {
            Err(ServiceError::MissingQueryParameter("test")) => true,
            _ => false,
        });
    }

    #[test]
    fn integer_query_parameter_should_error_if_the_value_is_not_an_unsigned_integer() {
        let url = Url::parse("http://127.0.0.1/?test=-1").unwrap();
        let result = integer_query_parameter::<u32>(&url, "test");

        assert!(match result {
            Err(ServiceError::QueryParameterParseError(_)) => true,
            _ => false,
        });
    }
}

use std::str::FromStr;

pub fn info(name: &str) -> Result<Vec<u8>, crate::Error> {
    let locale = wrap::Locale::from(icu_locid::Locale::from_str(name)?);
    let mut w = vec![];
    ciborium::into_writer(&locale, &mut w)?;

    Ok(w)
}

/// This module wraps [icu_locid::Locale] to be able to serialize it.
mod wrap {
    use serde::{ser::SerializeSeq, Serialize};

    #[derive(Serialize)]
    pub struct Locale {
        pub id: LanguageIdentifier,
        pub extensions: Extensions,
    }

    impl From<icu_locid::Locale> for Locale {
        fn from(value: icu_locid::Locale) -> Self {
            Self {
                id: value.id.into(),
                extensions: value.extensions.into(),
            }
        }
    }

    #[derive(Serialize)]
    pub struct LanguageIdentifier {
        pub language: Displayer<icu_locid::subtags::Language>,
        pub script: Option<Displayer<icu_locid::subtags::Script>>,
        pub region: Option<Displayer<icu_locid::subtags::Region>>,
        pub variants: Slicer<icu_locid::subtags::Variants>,
    }

    impl From<icu_locid::LanguageIdentifier> for LanguageIdentifier {
        fn from(value: icu_locid::LanguageIdentifier) -> Self {
            Self {
                language: value.language.into(),
                script: value.script.map(Into::into),
                region: value.region.map(Into::into),
                variants: value.variants.into(),
            }
        }
    }

    #[derive(Serialize)]
    pub struct Extensions {
        pub unicode: Unicode,
        pub transform: Transform,
        pub private: Slicer<icu_locid::extensions::private::Private>,
        pub other: Vec<Displayer<icu_locid::extensions::other::Other>>,
    }

    impl From<icu_locid::extensions::Extensions> for Extensions {
        fn from(value: icu_locid::extensions::Extensions) -> Self {
            Self {
                unicode: value.unicode.into(),
                transform: value.transform.into(),
                private: value.private.into(),
                other: value.other.into_iter().map(Into::into).collect(),
            }
        }
    }

    #[derive(Serialize)]
    pub struct Unicode {
        pub keywords: Displayer<icu_locid::extensions::unicode::Keywords>,
        pub attributes: Slicer<icu_locid::extensions::unicode::Attributes>,
    }

    impl From<icu_locid::extensions::unicode::Unicode> for Unicode {
        fn from(value: icu_locid::extensions::unicode::Unicode) -> Self {
            Self {
                keywords: value.keywords.into(),
                attributes: value.attributes.into(),
            }
        }
    }

    #[derive(Serialize)]
    pub struct Transform {
        pub lang: Option<LanguageIdentifier>,
        pub fields: Displayer<icu_locid::extensions::transform::Fields>,
    }

    impl From<icu_locid::extensions::transform::Transform> for Transform {
        fn from(value: icu_locid::extensions::transform::Transform) -> Self {
            Self {
                lang: value.lang.map(Into::into),
                fields: value.fields.into(),
            }
        }
    }

    #[repr(transparent)]
    pub struct Displayer<T>(pub T);

    impl<T: std::fmt::Display> Serialize for Displayer<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.collect_str(&self.0)
        }
    }

    impl<T> From<T> for Displayer<T> {
        fn from(value: T) -> Self {
            Self(value)
        }
    }

    #[repr(transparent)]
    pub struct Slicer<T>(pub T);

    impl<T> Serialize for Slicer<T>
    where
        T: SliceLike,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let slice = self.0.as_slice();
            let mut seq = serializer.serialize_seq(Some(slice.len()))?;
            for element in slice {
                seq.serialize_element(&Displayer(element))?;
            }
            seq.end()
        }
    }

    trait SliceLike {
        fn as_slice(&self) -> &[impl std::fmt::Display];
    }

    macro_rules! slice_like_for_deref {
        ($t:ty) => {
            impl SliceLike for $t {
                fn as_slice(&self) -> &[impl std::fmt::Display] {
                    std::ops::Deref::deref(self)
                }
            }
        };
    }

    slice_like_for_deref!(icu_locid::subtags::Variants);
    slice_like_for_deref!(icu_locid::extensions::unicode::Attributes);
    slice_like_for_deref!(icu_locid::extensions::private::Private);

    impl<T> From<T> for Slicer<T> {
        fn from(value: T) -> Self {
            Self(value)
        }
    }
}

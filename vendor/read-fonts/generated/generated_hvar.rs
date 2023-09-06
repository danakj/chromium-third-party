// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [HVAR (Horizontal Metrics Variations)](https://docs.microsoft.com/en-us/typography/opentype/spec/hvar) table
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct HvarMarker {}

impl HvarMarker {
    fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + MajorMinor::RAW_BYTE_LEN
    }
    fn item_variation_store_offset_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + Offset32::RAW_BYTE_LEN
    }
    fn advance_width_mapping_offset_byte_range(&self) -> Range<usize> {
        let start = self.item_variation_store_offset_byte_range().end;
        start..start + Offset32::RAW_BYTE_LEN
    }
    fn lsb_mapping_offset_byte_range(&self) -> Range<usize> {
        let start = self.advance_width_mapping_offset_byte_range().end;
        start..start + Offset32::RAW_BYTE_LEN
    }
    fn rsb_mapping_offset_byte_range(&self) -> Range<usize> {
        let start = self.lsb_mapping_offset_byte_range().end;
        start..start + Offset32::RAW_BYTE_LEN
    }
}

impl TopLevelTable for Hvar<'_> {
    /// `HVAR`
    const TAG: Tag = Tag::new(b"HVAR");
}

impl<'a> FontRead<'a> for Hvar<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<MajorMinor>();
        cursor.advance::<Offset32>();
        cursor.advance::<Offset32>();
        cursor.advance::<Offset32>();
        cursor.advance::<Offset32>();
        cursor.finish(HvarMarker {})
    }
}

/// The [HVAR (Horizontal Metrics Variations)](https://docs.microsoft.com/en-us/typography/opentype/spec/hvar) table
pub type Hvar<'a> = TableRef<'a, HvarMarker>;

impl<'a> Hvar<'a> {
    /// Major version number of the horizontal metrics variations table — set to 1.
    /// Minor version number of the horizontal metrics variations table — set to 0.
    pub fn version(&self) -> MajorMinor {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Offset in bytes from the start of this table to the item variation store table.
    pub fn item_variation_store_offset(&self) -> Offset32 {
        let range = self.shape.item_variation_store_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`item_variation_store_offset`][Self::item_variation_store_offset].
    pub fn item_variation_store(&self) -> Result<ItemVariationStore<'a>, ReadError> {
        let data = self.data;
        self.item_variation_store_offset().resolve(data)
    }

    /// Offset in bytes from the start of this table to the delta-set index mapping for advance widths (may be NULL).
    pub fn advance_width_mapping_offset(&self) -> Nullable<Offset32> {
        let range = self.shape.advance_width_mapping_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`advance_width_mapping_offset`][Self::advance_width_mapping_offset].
    pub fn advance_width_mapping(&self) -> Option<Result<DeltaSetIndexMap<'a>, ReadError>> {
        let data = self.data;
        self.advance_width_mapping_offset().resolve(data)
    }

    /// Offset in bytes from the start of this table to the delta-set index mapping for left side bearings (may be NULL).
    pub fn lsb_mapping_offset(&self) -> Nullable<Offset32> {
        let range = self.shape.lsb_mapping_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`lsb_mapping_offset`][Self::lsb_mapping_offset].
    pub fn lsb_mapping(&self) -> Option<Result<DeltaSetIndexMap<'a>, ReadError>> {
        let data = self.data;
        self.lsb_mapping_offset().resolve(data)
    }

    /// Offset in bytes from the start of this table to the delta-set index mapping for right side bearings (may be NULL).
    pub fn rsb_mapping_offset(&self) -> Nullable<Offset32> {
        let range = self.shape.rsb_mapping_offset_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Attempt to resolve [`rsb_mapping_offset`][Self::rsb_mapping_offset].
    pub fn rsb_mapping(&self) -> Option<Result<DeltaSetIndexMap<'a>, ReadError>> {
        let data = self.data;
        self.rsb_mapping_offset().resolve(data)
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for Hvar<'a> {
    fn type_name(&self) -> &str {
        "Hvar"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new(
                "item_variation_store_offset",
                FieldType::offset(
                    self.item_variation_store_offset(),
                    self.item_variation_store(),
                ),
            )),
            2usize => Some(Field::new(
                "advance_width_mapping_offset",
                FieldType::offset(
                    self.advance_width_mapping_offset(),
                    self.advance_width_mapping(),
                ),
            )),
            3usize => Some(Field::new(
                "lsb_mapping_offset",
                FieldType::offset(self.lsb_mapping_offset(), self.lsb_mapping()),
            )),
            4usize => Some(Field::new(
                "rsb_mapping_offset",
                FieldType::offset(self.rsb_mapping_offset(), self.rsb_mapping()),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for Hvar<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}
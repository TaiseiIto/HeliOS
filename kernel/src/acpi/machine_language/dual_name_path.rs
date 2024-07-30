use {
    alloc::{
        string::String,
        vec::Vec,
    },
    core::fmt,
    super::{
        DualNamePrefix,
        NameSeg,
        Reader,
    },
};

/// # DualNamePath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub struct DualNamePath {
    dual_name_prefix: DualNamePrefix,
    name_segs: [NameSeg; 2],
}

impl fmt::Debug for DualNamePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("DualNamePath");
        let Self {
            dual_name_prefix,
            name_segs,
        } = self;
        debug_tuple.field(dual_name_prefix);
        name_segs
            .as_slice()
            .iter()
            .for_each(|name_seg| {
                debug_tuple.field(name_seg);
            });
        debug_tuple.finish()
    }
}

impl From<&DualNamePath> for String {
    fn from(dual_name_path: &DualNamePath) -> Self {
        let dual_name_path: Vec<String> = dual_name_path
            .name_segs
            .as_slice()
            .iter()
            .map(|name_seg| name_seg.into())
            .collect();
        dual_name_path.concat()
    }
}

impl From<&[u8]> for DualNamePath {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (dual_name_prefix, aml): (DualNamePrefix, &[u8]) = DualNamePrefix::read(aml);
        let name_segs: Vec<NameSeg> = (0..2)
            .fold((Vec::new(), aml), |(mut name_segs, aml), _| {
                let (name_seg, aml): (NameSeg, &[u8]) = NameSeg::read(aml);
                name_segs.push(name_seg);
                (name_segs, aml)
            })
            .0;
        let name_segs: [NameSeg; 2] = name_segs
            .try_into()
            .unwrap();
        Self {
            dual_name_prefix,
            name_segs,
        }
    }
}

impl Reader<'_> for DualNamePath {
    fn length(&self) -> usize {
        let Self {
            dual_name_prefix,
            name_segs,
        } = self;
        dual_name_prefix.length() + name_segs
            .as_slice()
            .iter()
            .map(|name_seg| name_seg.length())
            .sum::<usize>()
    }

    fn matches(aml: &[u8]) -> bool {
        DualNamePrefix::matches(aml)
    }
}


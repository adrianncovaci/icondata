//! This crate provides a collection of icons in the form of SVG data
//! and an enum to select them. It re-exports all icons from the icondata_* crates.
//!
//! This crate is meant to be used to build component libraries for web frameworks.
//! To do so, an [`Icon`] enum is provided, which can be used to select any icon from the
//! collection. This enum is marked as non_exhaustive, as new icon libraries may be added in the future.
//!
//! The [`Icon`] enum can be converted into an [`IconData`] struct, which contains the SVG data.
//!
//!
pub use icondata_core::IconData;

#[cfg(feature = "Ai")]
pub use icondata_ai::*;
#[cfg(feature = "Bi")]
pub use icondata_bi::*;
#[cfg(feature = "Bs")]
pub use icondata_bs::*;
#[cfg(feature = "Cg")]
pub use icondata_cg::*;
#[cfg(feature = "Ch")]
pub use icondata_ch::*;
#[cfg(feature = "Fa")]
pub use icondata_fa::*;
#[cfg(feature = "Fi")]
pub use icondata_fi::*;
#[cfg(feature = "Hi")]
pub use icondata_hi::*;
#[cfg(feature = "Im")]
pub use icondata_im::*;
#[cfg(feature = "Io")]
pub use icondata_io::*;
#[cfg(feature = "Lu")]
pub use icondata_lu::*;
#[cfg(feature = "Oc")]
pub use icondata_oc::*;
#[cfg(feature = "Ri")]
pub use icondata_ri::*;
#[cfg(feature = "Si")]
pub use icondata_si::*;
#[cfg(feature = "Tb")]
pub use icondata_tb::*;
#[cfg(feature = "Ti")]
pub use icondata_ti::*;
#[cfg(feature = "Vs")]
pub use icondata_vs::*;
#[cfg(feature = "Wi")]
pub use icondata_wi::*;

#[cfg(feature = "macros")]
pub use icondata_macros::*;

/// The main enum to select an icon. This enum contains all icons from icondata_* libraries, and
/// implements [`From`] for [`IconData`], so it can be converted into an [`IconData`] struct.
/// It also implements [`From`] for any icon in any icondata_* library.
///
/// This enum is marked as non_exhaustive, as new icon libraries may be added in the future.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Icon {
    #[cfg(feature = "Ai")]
    Ai(AiIcon),
    #[cfg(feature = "Bi")]
    Bi(BiIcon),
    #[cfg(feature = "Bs")]
    Bs(BsIcon),
    #[cfg(feature = "Cg")]
    Cg(CgIcon),
    #[cfg(feature = "Ch")]
    Ch(ChIcon),
    #[cfg(feature = "Fa")]
    Fa(FaIcon),
    #[cfg(feature = "Fi")]
    Fi(FiIcon),
    #[cfg(feature = "Hi")]
    Hi(HiIcon),
    #[cfg(feature = "Im")]
    Im(ImIcon),
    #[cfg(feature = "Io")]
    Io(IoIcon),
    #[cfg(feature = "Lu")]
    Lu(LuIcon),
    #[cfg(feature = "Oc")]
    Oc(OcIcon),
    #[cfg(feature = "Ri")]
    Ri(RiIcon),
    #[cfg(feature = "Si")]
    Si(SiIcon),
    #[cfg(feature = "Tb")]
    Tb(TbIcon),
    #[cfg(feature = "Ti")]
    Ti(TiIcon),
    #[cfg(feature = "Vs")]
    Vs(VsIcon),
    #[cfg(feature = "Wi")]
    Wi(WiIcon),
    Custom(CustomIcon),
}
impl From<Icon> for icondata_core::IconData {
    fn from(icon: Icon) -> Self {
        match icon {
            #[cfg(feature = "Ai")]
            Icon::Ai(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Bi")]
            Icon::Bi(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Bs")]
            Icon::Bs(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Cg")]
            Icon::Cg(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Ch")]
            Icon::Ch(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Fa")]
            Icon::Fa(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Fi")]
            Icon::Fi(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Hi")]
            Icon::Hi(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Im")]
            Icon::Im(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Io")]
            Icon::Io(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Lu")]
            Icon::Lu(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Oc")]
            Icon::Oc(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Ri")]
            Icon::Ri(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Si")]
            Icon::Si(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Tb")]
            Icon::Tb(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Ti")]
            Icon::Ti(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Vs")]
            Icon::Vs(icon) => icondata_core::IconData::from(icon),
            #[cfg(feature = "Wi")]
            Icon::Wi(icon) => icondata_core::IconData::from(icon),
            Icon::Custom(icon) => icondata_core::IconData::from(icon),
        }
    }
}

#[cfg(feature = "Ai")]
impl From<AiIcon> for Icon {
    fn from(icon: AiIcon) -> Self {
        Self::Ai(icon)
    }
}

#[cfg(feature = "Bi")]
impl From<BiIcon> for Icon {
    fn from(icon: BiIcon) -> Self {
        Self::Bi(icon)
    }
}

#[cfg(feature = "Bs")]
impl From<BsIcon> for Icon {
    fn from(icon: BsIcon) -> Self {
        Self::Bs(icon)
    }
}

#[cfg(feature = "Cg")]
impl From<CgIcon> for Icon {
    fn from(icon: CgIcon) -> Self {
        Self::Cg(icon)
    }
}

#[cfg(feature = "Ch")]
impl From<ChIcon> for Icon {
    fn from(icon: ChIcon) -> Self {
        Self::Ch(icon)
    }
}

#[cfg(feature = "Fa")]
impl From<FaIcon> for Icon {
    fn from(icon: FaIcon) -> Self {
        Self::Fa(icon)
    }
}

#[cfg(feature = "Fi")]
impl From<FiIcon> for Icon {
    fn from(icon: FiIcon) -> Self {
        Self::Fi(icon)
    }
}

#[cfg(feature = "Hi")]
impl From<HiIcon> for Icon {
    fn from(icon: HiIcon) -> Self {
        Self::Hi(icon)
    }
}

#[cfg(feature = "Im")]
impl From<ImIcon> for Icon {
    fn from(icon: ImIcon) -> Self {
        Self::Im(icon)
    }
}

#[cfg(feature = "Io")]
impl From<IoIcon> for Icon {
    fn from(icon: IoIcon) -> Self {
        Self::Io(icon)
    }
}

#[cfg(feature = "Lu")]
impl From<LuIcon> for Icon {
    fn from(icon: LuIcon) -> Self {
        Self::Lu(icon)
    }
}

#[cfg(feature = "Oc")]
impl From<OcIcon> for Icon {
    fn from(icon: OcIcon) -> Self {
        Self::Oc(icon)
    }
}

#[cfg(feature = "Ri")]
impl From<RiIcon> for Icon {
    fn from(icon: RiIcon) -> Self {
        Self::Ri(icon)
    }
}

#[cfg(feature = "Si")]
impl From<SiIcon> for Icon {
    fn from(icon: SiIcon) -> Self {
        Self::Si(icon)
    }
}

#[cfg(feature = "Tb")]
impl From<TbIcon> for Icon {
    fn from(icon: TbIcon) -> Self {
        Self::Tb(icon)
    }
}

#[cfg(feature = "Ti")]
impl From<TiIcon> for Icon {
    fn from(icon: TiIcon) -> Self {
        Self::Ti(icon)
    }
}

#[cfg(feature = "Vs")]
impl From<VsIcon> for Icon {
    fn from(icon: VsIcon) -> Self {
        Self::Vs(icon)
    }
}

#[cfg(feature = "Wi")]
impl From<WiIcon> for Icon {
    fn from(icon: WiIcon) -> Self {
        Self::Wi(icon)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "strum", derive(strum::EnumIter, strum::EnumVariantNames))]
pub struct CustomIcon {
    pub style: Option<&'static str>,
    pub x: Option<&'static str>,
    pub y: Option<&'static str>,
    pub width: Option<&'static str>,
    pub height: Option<&'static str>,
    pub view_box: Option<&'static str>,
    pub stroke_linecap: Option<&'static str>,
    pub stroke_linejoin: Option<&'static str>,
    pub stroke_width: Option<&'static str>,
    pub stroke: Option<&'static str>,
    pub fill: Option<&'static str>,
    pub data: &'static str,
}

impl From<CustomIcon> for icondata_core::IconData {
    fn from(icon: CustomIcon) -> icondata_core::IconData {
        Self {
            style: icon.style,
            x: icon.x,
            y: icon.y,
            width: icon.width,
            height: icon.height,
            view_box: icon.view_box,
            stroke_linecap: icon.stroke_linecap,
            stroke_linejoin: icon.stroke_linejoin,
            stroke_width: icon.stroke_width,
            stroke: icon.stroke,
            fill: icon.fill,
            data: icon.data,
        }
    }
}

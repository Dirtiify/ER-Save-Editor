use std::{
    collections::HashMap,
    sync::OnceLock,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum MapName {
    AcademyOfRayaLucaria,
    AinselRiver,
    AinselRiverMain,
    AltusPlateau,
    BellumHighway,
    Caelid,
    CapitalOutskirts,
    ConsecratedSnowfield,
    CrumblingFarumAzula,
    DeeprootDepths,
    ElphaelBraceOfTheHaligtree,
    FlamePeak,
    ForbiddenLands,
    GreyollsDragonbarrow,
    LakeOfRot,
    LeyndellAshenCapital,
    LeyndellRoyalCapital,
    Limgrave,
    LiurniaOfTheLakes,
    MiquellasHaligtree,
    MohgwynPalace,
    MoonlightAltar,
    MountaintopsOfTheGiants,
    MtGelmir,
    NokronEternalCity,
    RoundtableHold,
    RuinStrewnPrecipice,
    SiofraRiver,
    StonePlatform,
    Stormhill,
    StormveilCastle,
    StrandedGraveyard,
    SubterraneanShunningGrounds,
    SwampOfAeonia,
    VolcanoManor,
    WeepingPeninsula,

    //DLC
    RealmofShadow,
}

impl MapName {
    #[rustfmt::skip]
    pub fn map_names() -> &'static HashMap<MapName, &'static str> {
        static MAP_NAMES: OnceLock<HashMap<MapName, &'static str>> = OnceLock::new();

        MAP_NAMES.get_or_init(||
            HashMap::from([
                (MapName::RoundtableHold, "Table of Lost Grace / Roundtable Hold"),
                (MapName::Limgrave, "Limgrave"),
                (MapName::StrandedGraveyard, "Stranded Graveyard"),
                (MapName::Stormhill, "Stormhill"),
                (MapName::WeepingPeninsula, "Weeping Peninsula"),
                (MapName::StormveilCastle, "Stormveil Castle"),
                (MapName::LiurniaOfTheLakes, "Liurnia of the Lakes"),
                (MapName::BellumHighway, "Bellum Highway"),
                (MapName::RuinStrewnPrecipice, "Ruin-Strewn Precipice"),
                (MapName::MoonlightAltar, "Moonlight Altar"),
                (MapName::AcademyOfRayaLucaria, "Academy of Raya Lucaria"),
                (MapName::AltusPlateau, "Altus Plateau"),
                (MapName::MtGelmir, "Mt. Gelmir"),
                (MapName::CapitalOutskirts, "Capital Outskirts"),
                (MapName::VolcanoManor, "Volcano Manor"),
                (MapName::LeyndellRoyalCapital, "Leyndell, Royal Capital"),
                (MapName::SubterraneanShunningGrounds, "Subterranean Shunning-Grounds"),
                (MapName::LeyndellAshenCapital, "Leyndell, Ashen Capital"),
                (MapName::StonePlatform, "Stone Platform"),
                (MapName::Caelid, "Caelid"),
                (MapName::SwampOfAeonia, "Swamp of Aeonia"),
                (MapName::GreyollsDragonbarrow, "Greyoll's Dragonbarrow"),
                (MapName::ForbiddenLands, "Forbidden Lands"),
                (MapName::MountaintopsOfTheGiants, "Mountaintops of the Giants"),
                (MapName::FlamePeak, "Flame Peak"),
                (MapName::ConsecratedSnowfield, "Consecrated Snowfield"),
                (MapName::MiquellasHaligtree, "Miquella's Haligtree"),
                (MapName::ElphaelBraceOfTheHaligtree, "Elphael, Brace of the Haligtree"),
                (MapName::AinselRiver, "Ainsel River"),
                (MapName::AinselRiverMain, "Ainsel River Main"),
                (MapName::LakeOfRot, "Lake of Rot"),
                (MapName::NokronEternalCity, "Nokron, Eternal City"),
                (MapName::MohgwynPalace, "Mohgwyn Palace"),
                (MapName::SiofraRiver, "Siofra River"),
                (MapName::DeeprootDepths, "Deeproot Depths"),
                (MapName::CrumblingFarumAzula, "Crumbling Farum Azula"),
                
                // DLC
                (MapName::RealmofShadow, "Realm of Shadow (DLC)"),
            ])
        )
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Aar,
    Abk,
    Ace,
    Ach,
    Ada,
    Ady,
    Afa,
    Afh,
    Afr,
    Ain,
    Aka,
    Akk,
    Ale,
    Alg,
    Alt,
    Amh,
    Ang,
    Anp,
    Apa,
    Ara,
    Arc,
    Arg,
    Arn,
    Arp,
    Art,
    Arw,
    Asm,
    Ast,
    Ath,
    Aus,
    Ava,
    Ave,
    Awa,
    Aym,
    Aze,
    Bad,
    Bai,
    Bak,
    Bal,
    Bam,
    Ban,
    Bas,
    Bat,
    Bej,
    Bel,
    Bem,
    Ben,
    Ber,
    Bho,
    Bih,
    Bik,
    Bin,
    Bis,
    Bla,
    Bnt,
    Bod,
    Bos,
    Bra,
    Bre,
    Btk,
    Bua,
    Bug,
    Bul,
    Byn,
    Cad,
    Cai,
    Car,
    Cat,
    Cau,
    Ceb,
    Cel,
    Ces,
    Cha,
    Chb,
    Che,
    Chg,
    Chk,
    Chm,
    Chn,
    Cho,
    Chp,
    Chr,
    Chu,
    Chv,
    Chy,
    Cmc,
    Cnr,
    Cop,
    Cor,
    Cos,
    Cpe,
    Cpf,
    Cpp,
    Cre,
    Crh,
    Crp,
    Csb,
    Cus,
    Cym,
    Dak,
    Dan,
    Dar,
    Day,
    Del,
    Den,
    Deu,
    Dgr,
    Din,
    Div,
    Doi,
    Dra,
    Dsb,
    Dua,
    Dum,
    Dyu,
    Dzo,
    Efi,
    Egy,
    Eka,
    Ell,
    Elx,
    Eng,
    Enm,
    Epo,
    Est,
    Eus,
    Ewe,
    Ewo,
    Fan,
    Fao,
    Fas,
    Fat,
    Fij,
    Fil,
    Fin,
    Fiu,
    Fon,
    Fra,
    Frm,
    Fro,
    Frr,
    Frs,
    Fry,
    Ful,
    Fur,
    Gaa,
    Gay,
    Gba,
    Gem,
    Gez,
    Gil,
    Gla,
    Gle,
    Glg,
    Glv,
    Gmh,
    Goh,
    Gon,
    Gor,
    Got,
    Grb,
    Grc,
    Grn,
    Gsw,
    Guj,
    Gwi,
    Hai,
    Hat,
    Hau,
    Haw,
    Heb,
    Her,
    Hil,
    Him,
    Hin,
    Hit,
    Hmn,
    Hmo,
    Hrv,
    Hsb,
    Hun,
    Hup,
    Hye,
    Iba,
    Ibo,
    Ido,
    Iii,
    Ijo,
    Iku,
    Ile,
    Ilo,
    Ina,
    Inc,
    Ind,
    Ine,
    Inh,
    Ipk,
    Ira,
    Iro,
    Isl,
    Ita,
    Jav,
    Jbo,
    Jpn,
    Jpr,
    Jrb,
    Kaa,
    Kab,
    Kac,
    Kal,
    Kam,
    Kan,
    Kar,
    Kas,
    Kat,
    Kau,
    Kaw,
    Kaz,
    Kbd,
    Kha,
    Khi,
    Khm,
    Kho,
    Kik,
    Kin,
    Kir,
    Kmb,
    Kok,
    Kom,
    Kon,
    Kor,
    Kos,
    Kpe,
    Krc,
    Krl,
    Kro,
    Kru,
    Kua,
    Kum,
    Kur,
    Kut,
    Lad,
    Lah,
    Lam,
    Lao,
    Lat,
    Lav,
    Lez,
    Lim,
    Lin,
    Lit,
    Lol,
    Loz,
    Ltz,
    Lua,
    Lub,
    Lug,
    Lui,
    Lun,
    Luo,
    Lus,
    Mad,
    Mag,
    Mah,
    Mai,
    Mak,
    Mal,
    Man,
    Map,
    Mar,
    Mas,
    Mdf,
    Mdr,
    Men,
    Mga,
    Mic,
    Min,
    Mis,
    Mkd,
    Mkh,
    Mlg,
    Mlt,
    Mnc,
    Mni,
    Mno,
    Moh,
    Mon,
    Mos,
    Mri,
    Msa,
    Mul,
    Mun,
    Mus,
    Mwl,
    Mwr,
    Mya,
    Myn,
    Myv,
    Nah,
    Nai,
    Nap,
    Nau,
    Nav,
    Nbl,
    Nde,
    Ndo,
    Nds,
    Nep,
    New,
    Nia,
    Nic,
    Niu,
    Nld,
    Nno,
    Nob,
    Nog,
    Non,
    Nor,
    Nqo,
    Nso,
    Nub,
    Nwc,
    Nya,
    Nym,
    Nyn,
    Nyo,
    Nzi,
    Oci,
    Oji,
    Ori,
    Orm,
    Osa,
    Oss,
    Ota,
    Oto,
    Paa,
    Pag,
    Pal,
    Pam,
    Pan,
    Pap,
    Pau,
    Peo,
    Phi,
    Phn,
    Pli,
    Pol,
    Pon,
    Por,
    Pra,
    Pro,
    Pus,
    Qaa,
    Que,
    Raj,
    Rap,
    Rar,
    Roa,
    Roh,
    Rom,
    Ron,
    Run,
    Rup,
    Rus,
    Sad,
    Sag,
    Sah,
    Sai,
    Sal,
    Sam,
    San,
    Sas,
    Sat,
    Scn,
    Sco,
    Sel,
    Sem,
    Sga,
    Sgn,
    Shn,
    Sid,
    Sin,
    Sio,
    Sit,
    Sla,
    Slk,
    Slv,
    Sma,
    Sme,
    Smi,
    Smj,
    Smn,
    Smo,
    Sms,
    Sna,
    Snd,
    Snk,
    Sog,
    Som,
    Son,
    Sot,
    Spa,
    Sqi,
    Srd,
    Srn,
    Srp,
    Srr,
    Ssa,
    Ssw,
    Suk,
    Sun,
    Sus,
    Sux,
    Swa,
    Swe,
    Syc,
    Syr,
    Tah,
    Tai,
    Tam,
    Tat,
    Tel,
    Tem,
    Ter,
    Tet,
    Tgk,
    Tgl,
    Tha,
    Tig,
    Tir,
    Tiv,
    Tkl,
    Tlh,
    Tli,
    Tmh,
    Tog,
    Ton,
    Tpi,
    Tsi,
    Tsn,
    Tso,
    Tuk,
    Tum,
    Tup,
    Tur,
    Tut,
    Tvl,
    Twi,
    Tyv,
    Udm,
    Uga,
    Uig,
    Ukr,
    Umb,
    Und,
    Urd,
    Uzb,
    Vai,
    Vec,
    Ven,
    Vie,
    Vol,
    Vot,
    Wak,
    Wal,
    War,
    Was,
    Wen,
    Wln,
    Wol,
    Xal,
    Xho,
    Yao,
    Yap,
    Yid,
    Yor,
    Ypk,
    Zap,
    Zbl,
    Zen,
    Zgh,
    Zha,
    Zho,
    Znd,
    Zul,
    Zun,
    Zxx,
    Zza,
}

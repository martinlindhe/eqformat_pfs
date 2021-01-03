
use crate::pfs::PFSArchive;

#[test]
fn can_parse_s3d() {
    let s3d = PFSArchive::from_file("samples/chardok_2_obj.s3d").unwrap();

    assert_eq!(5, s3d.files.len());
    assert_eq!("chardok_2_obj.wld", s3d.files[0].name);
    assert_eq!("charlev01.dds", s3d.files[1].name);
    assert_eq!("charlev02.dds", s3d.files[2].name);
    assert_eq!("charlev03.dds", s3d.files[3].name);
    assert_eq!("palette.bmp", s3d.files[4].name);

    let wld_entry = s3d.get_entry("chardok_2_obj.wld").unwrap();
    assert_eq!("chardok_2_obj.wld", wld_entry.name);

    let wld_default = s3d.default_wld().unwrap();
    assert_eq!("chardok_2_obj.wld", wld_default.name);
}

#[test]
fn can_parse_s3d_order() {
    let s3d = PFSArchive::from_file("samples/butcher2_chr.s3d").unwrap();

    assert_eq!(9, s3d.files.len());
    assert_eq!("butcher2_chr.wld", s3d.files[8].name);
    assert_eq!(0x6FAB5958, s3d.files[8].crc);
}

#[test]
fn can_parse_eqg() {
    let s3d = PFSArchive::from_file("samples/guildhalldoor.eqg").unwrap();

    assert_eq!(1, s3d.files.len());
    assert_eq!("obj_guild_door_switch.mod", s3d.files[0].name);
}

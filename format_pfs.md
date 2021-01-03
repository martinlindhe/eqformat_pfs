# PFS file format

The file format called "PFS" in this document is an archive format used in Everquest.
Extensions: .s3d, .eqg, .pfs, .pak

Every PFS has a "directory" entry, that contains the filenames


A .s3d file describes a zone and embeds a .wld file
A .eqg usually has only one file with a "EQGM" header


offset              size        notes
              0     u32         ofs_entries     offset to file entries
              4     u32         magic 0x20534650 ("PFS ")

ofs_entries + 0     u32         number of entries
            + 4     u32         crc  (of compressed data ?) if 0x61580AC9 then it's a directory entry
            + 8     u32         ofs_file        offset of compressed data
            + c     u32         expanded_size   size of expanded data

ofs_file    + 0     u32         len_comp        compressed length
            + 4     u32         expanded length
            + 8     len_comp    compressed data block (usually 0x2000 bytes)
                                multiple compressed blocks (extract until buffer len is expanded_size)

ofs_dir     + 0     u32         entry length
            + 4     u32         fname_len
            + 8     fname_len   0-padded file name



ssratemple.s3d - the level map is embedded as ssratemple.wld
ssratemple_chr.s3d - zone specific models ?
ssratemple_obj.s3d - zone objs ?



# Credits

https://github.com/alimalkhalifa/VisualEQ/blob/master/src/server/loaders/s3d.js

const END_TAG_LEN: u32 = 8;
const HEADER_LEN: u32 = 24;
pub const MAGIC: u32 = 0xe85250d6;

#[repr(u32)]
pub enum HeaderArch {
	I386 = 0, 
	Mips = 4
}

#[repr(C)]
pub struct Header {
	pub magic: u32, 
	pub arch: HeaderArch, pub 
	header_length: u32, 
	pub checksum: u32, 
	pub end_tag: Tag
}

#[repr(C)]
pub struct Info { 
	pub length: u32, 
	_pad: u32, 
	tag_start: Tag
}

#[repr(C)]
#[derive(Debug)]
pub struct Tag {
	pub ty: TagType, 
	length: u32
}

#[repr(u32)]
#[derive(Debug, Eq, PartialEq)]
pub enum TagType {
    	End              = 0,
    	CommandLine      = 1, 
	BootloaderName   = 2, 
	Modules          = 3, 
	BasicMemInfo     = 4, 
	BIOSBootDev      = 5, 
	MemoryMap        = 6, 
	VBEInfo          = 7, 
	FramebufferInfo  = 8,
	ELFSections      = 9, 
	APMTable         = 10
}

#[linkage = "external"]
#[link_section = ".multiboot_header"]
pub static HEADER: Header = Header {
	magic: MAGIC, 
	arch: HeaderArch::I386, 
	header_length: HEADER_LEN, 
	checksum: -((MAGIC + 0 + HEADER_LEN) as i32) as u32, 
	end_tag: Tag { 	
	ty: TagType::End, 
	length: END_TAG_LEN
    }
};


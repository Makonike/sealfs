// Copyright 2022 labring. All rights reserved.
//
// SPDX-License-Identifier: Apache-2.0

use fuser::{FileAttr, FileType};
use libc::{
    stat, statx, statx_timestamp, S_IFBLK, S_IFCHR, S_IFDIR, S_IFIFO, S_IFLNK, S_IFREG, S_IFSOCK,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::{
    collections::BTreeMap,
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

#[macro_export]
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        //  Undefined Behavior: dereferences a null pointer.
        //  Undefined Behavior: accesses field outside of valid memory area.
        unsafe { &(*(0 as *const $ty)).$field as *const _ as usize - 1 }
    };
}

pub enum OperationType {
    Unkown = 0,
    Lookup = 1,
    CreateFile = 2,
    CreateDir = 3,
    GetFileAttr = 4,
    ReadDir = 5,
    OpenFile = 6,
    ReadFile = 7,
    WriteFile = 8,
    DeleteFile = 9,
    DeleteDir = 10,
    DirectoryAddEntry = 11,
    DirectoryDeleteEntry = 12,
    TruncateFile = 13,
    CheckFile = 14,
    CheckDir = 15,
    CreateDirNoParent = 16,
    CreateFileNoParent = 17,
    DeleteDirNoParent = 18,
    DeleteFileNoParent = 19,
    CreateVolume = 20,
    InitVolume = 21,
    ListVolumes = 22,
    DeleteVolume = 23,
    CleanVolume = 24,
}

impl TryFrom<u32> for OperationType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OperationType::Unkown),
            1 => Ok(OperationType::Lookup),
            2 => Ok(OperationType::CreateFile),
            3 => Ok(OperationType::CreateDir),
            4 => Ok(OperationType::GetFileAttr),
            5 => Ok(OperationType::ReadDir),
            6 => Ok(OperationType::OpenFile),
            7 => Ok(OperationType::ReadFile),
            8 => Ok(OperationType::WriteFile),
            9 => Ok(OperationType::DeleteFile),
            10 => Ok(OperationType::DeleteDir),
            11 => Ok(OperationType::DirectoryAddEntry),
            12 => Ok(OperationType::DirectoryDeleteEntry),
            13 => Ok(OperationType::TruncateFile),
            14 => Ok(OperationType::CheckFile),
            15 => Ok(OperationType::CheckDir),
            16 => Ok(OperationType::CreateDirNoParent),
            17 => Ok(OperationType::CreateFileNoParent),
            18 => Ok(OperationType::DeleteDirNoParent),
            19 => Ok(OperationType::DeleteFileNoParent),
            20 => Ok(OperationType::CreateVolume),
            21 => Ok(OperationType::InitVolume),
            22 => Ok(OperationType::ListVolumes),
            23 => Ok(OperationType::DeleteVolume),
            24 => Ok(OperationType::CleanVolume),
            _ => panic!("Unkown value: {}", value),
        }
    }
}

impl From<OperationType> for u32 {
    fn from(value: OperationType) -> Self {
        match value {
            OperationType::Unkown => 0,
            OperationType::Lookup => 1,
            OperationType::CreateFile => 2,
            OperationType::CreateDir => 3,
            OperationType::GetFileAttr => 4,
            OperationType::ReadDir => 5,
            OperationType::OpenFile => 6,
            OperationType::ReadFile => 7,
            OperationType::WriteFile => 8,
            OperationType::DeleteFile => 9,
            OperationType::DeleteDir => 10,
            OperationType::DirectoryAddEntry => 11,
            OperationType::DirectoryDeleteEntry => 12,
            OperationType::TruncateFile => 13,
            OperationType::CheckFile => 14,
            OperationType::CheckDir => 15,
            OperationType::CreateDirNoParent => 16,
            OperationType::CreateFileNoParent => 17,
            OperationType::DeleteDirNoParent => 18,
            OperationType::DeleteFileNoParent => 19,
            OperationType::CreateVolume => 20,
            OperationType::InitVolume => 21,
            OperationType::ListVolumes => 22,
            OperationType::DeleteVolume => 23,
            OperationType::CleanVolume => 24,
        }
    }
}

pub enum ManagerOperationType {
    SendHeart = 101,
    GetMetadata = 102,
    GetClusterStatus = 103,
    GetHashRing = 104,
    GetNewHashRing = 105,
    AddNodes = 106,
    RemoveNodes = 107,
    UpdateServerStatus = 108,
    FinishServer = 109,
}

impl TryFrom<u32> for ManagerOperationType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            101 => Ok(ManagerOperationType::SendHeart),
            102 => Ok(ManagerOperationType::GetMetadata),
            103 => Ok(ManagerOperationType::GetClusterStatus),
            104 => Ok(ManagerOperationType::GetHashRing),
            105 => Ok(ManagerOperationType::GetNewHashRing),
            106 => Ok(ManagerOperationType::AddNodes),
            107 => Ok(ManagerOperationType::RemoveNodes),
            108 => Ok(ManagerOperationType::UpdateServerStatus),
            109 => Ok(ManagerOperationType::FinishServer),
            _ => panic!("Unkown value: {}", value),
        }
    }
}

impl From<ManagerOperationType> for u32 {
    fn from(value: ManagerOperationType) -> Self {
        match value {
            ManagerOperationType::SendHeart => 101,
            ManagerOperationType::GetMetadata => 102,
            ManagerOperationType::GetClusterStatus => 103,
            ManagerOperationType::GetHashRing => 104,
            ManagerOperationType::GetNewHashRing => 105,
            ManagerOperationType::AddNodes => 106,
            ManagerOperationType::RemoveNodes => 107,
            ManagerOperationType::UpdateServerStatus => 108,
            ManagerOperationType::FinishServer => 109,
        }
    }
}

impl ManagerOperationType {
    pub fn to_le_bytes(&self) -> [u8; 4] {
        match self {
            ManagerOperationType::SendHeart => 101u32.to_le_bytes(),
            ManagerOperationType::GetMetadata => 102u32.to_le_bytes(),
            ManagerOperationType::GetClusterStatus => 103u32.to_le_bytes(),
            ManagerOperationType::GetHashRing => 104u32.to_le_bytes(),
            ManagerOperationType::GetNewHashRing => 105u32.to_le_bytes(),
            ManagerOperationType::AddNodes => 106u32.to_le_bytes(),
            ManagerOperationType::RemoveNodes => 107u32.to_le_bytes(),
            ManagerOperationType::UpdateServerStatus => 108u32.to_le_bytes(),
            ManagerOperationType::FinishServer => 109u32.to_le_bytes(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ServerType {
    Running = 1,
    Add = 2,
    Remove = 3,
}

impl TryFrom<u32> for ServerType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ServerType::Running),
            2 => Ok(ServerType::Add),
            3 => Ok(ServerType::Remove),
            _ => panic!("Unkown value: {}", value),
        }
    }
}

impl From<ServerType> for u32 {
    fn from(value: ServerType) -> Self {
        match value {
            ServerType::Running => 1,
            ServerType::Add => 2,
            ServerType::Remove => 3,
        }
    }
}

impl Display for ServerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerType::Running => write!(f, "Running"),
            ServerType::Add => write!(f, "Add"),
            ServerType::Remove => write!(f, "Remove"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ServerStatus {
    Initializing = 201,
    PreTransfer = 202,
    Transferring = 203,
    PreFinish = 204,
    Finishing = 205,
    Finished = 206,
}

impl TryFrom<u32> for ServerStatus {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            201 => Ok(ServerStatus::Initializing),
            202 => Ok(ServerStatus::PreTransfer),
            203 => Ok(ServerStatus::Transferring),
            204 => Ok(ServerStatus::PreFinish),
            205 => Ok(ServerStatus::Finishing),
            206 => Ok(ServerStatus::Finished),
            _ => Err(format!("Unkown value: {}", value)),
        }
    }
}

impl From<ServerStatus> for u32 {
    fn from(value: ServerStatus) -> Self {
        match value {
            ServerStatus::Initializing => 201,
            ServerStatus::PreTransfer => 202,
            ServerStatus::Transferring => 203,
            ServerStatus::PreFinish => 204,
            ServerStatus::Finishing => 205,
            ServerStatus::Finished => 206,
        }
    }
}

impl Display for ServerStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initializing => write!(f, "Init"),
            Self::PreTransfer => write!(f, "PreTransfer"),
            Self::Transferring => write!(f, "Transferring"),
            Self::PreFinish => write!(f, "PreFinish"),
            Self::Finishing => write!(f, "Finish"),
            Self::Finished => write!(f, "CloseNodes"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClusterStatus {
    Initializing = 300,
    Idle = 301,
    NodesStarting = 302,
    SyncNewHashRing = 303,
    PreTransfer = 304,
    Transferring = 305,
    PreFinish = 306,
    Finishing = 307,
    StatusError = 308,
}

impl TryFrom<u32> for ClusterStatus {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            300 => Ok(ClusterStatus::Initializing),
            301 => Ok(ClusterStatus::Idle),
            302 => Ok(ClusterStatus::NodesStarting),
            303 => Ok(ClusterStatus::SyncNewHashRing),
            304 => Ok(ClusterStatus::PreTransfer),
            305 => Ok(ClusterStatus::Transferring),
            306 => Ok(ClusterStatus::PreFinish),
            307 => Ok(ClusterStatus::Finishing),
            308 => Ok(ClusterStatus::StatusError),
            _ => Err(format!("Unkown value: {}", value)),
        }
    }
}

impl From<ClusterStatus> for u32 {
    fn from(value: ClusterStatus) -> Self {
        match value {
            ClusterStatus::Initializing => 300,
            ClusterStatus::Idle => 301,
            ClusterStatus::NodesStarting => 302,
            ClusterStatus::SyncNewHashRing => 303,
            ClusterStatus::PreTransfer => 304,
            ClusterStatus::Transferring => 305,
            ClusterStatus::PreFinish => 306,
            ClusterStatus::Finishing => 307,
            ClusterStatus::StatusError => 308,
        }
    }
}

impl TryFrom<i32> for ClusterStatus {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            300 => Ok(ClusterStatus::Initializing),
            301 => Ok(ClusterStatus::Idle),
            302 => Ok(ClusterStatus::NodesStarting),
            303 => Ok(ClusterStatus::SyncNewHashRing),
            304 => Ok(ClusterStatus::PreTransfer),
            305 => Ok(ClusterStatus::Transferring),
            306 => Ok(ClusterStatus::PreFinish),
            307 => Ok(ClusterStatus::Finishing),
            308 => Ok(ClusterStatus::StatusError),
            _ => Err(format!("Unkown value: {}", value)),
        }
    }
}

impl From<ClusterStatus> for i32 {
    fn from(value: ClusterStatus) -> Self {
        match value {
            ClusterStatus::Initializing => 300,
            ClusterStatus::Idle => 301,
            ClusterStatus::NodesStarting => 302,
            ClusterStatus::SyncNewHashRing => 303,
            ClusterStatus::PreTransfer => 304,
            ClusterStatus::Transferring => 305,
            ClusterStatus::PreFinish => 306,
            ClusterStatus::Finishing => 307,
            ClusterStatus::StatusError => 308,
        }
    }
}

impl Display for ClusterStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initializing => write!(f, "Init"),
            Self::Idle => write!(f, "Idle"),
            Self::NodesStarting => write!(f, "AddNodes"),
            Self::SyncNewHashRing => write!(f, "SyncNewHashRing"),
            Self::PreTransfer => write!(f, "PreTransfer"),
            Self::Transferring => write!(f, "Transferring"),
            Self::PreFinish => write!(f, "PreFinish"),
            Self::Finishing => write!(f, "DeleteNodes"),
            Self::StatusError => write!(f, "StatusError"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileTypeSimple {
    RegularFile = 0,
    NamedPipe = 1,
    CharDevice = 2,
    BlockDevice = 3,
    Directory = 4,
    Symlink = 5,
    Socket = 6,
}

impl From<FileTypeSimple> for FileType {
    fn from(value: FileTypeSimple) -> Self {
        match value {
            FileTypeSimple::RegularFile => FileType::RegularFile,
            FileTypeSimple::NamedPipe => FileType::NamedPipe,
            FileTypeSimple::CharDevice => FileType::CharDevice,
            FileTypeSimple::BlockDevice => FileType::BlockDevice,
            FileTypeSimple::Directory => FileType::Directory,
            FileTypeSimple::Symlink => FileType::Symlink,
            FileTypeSimple::Socket => FileType::Socket,
        }
    }
}

impl From<FileType> for FileTypeSimple {
    fn from(value: FileType) -> Self {
        match value {
            FileType::RegularFile => FileTypeSimple::RegularFile,
            FileType::NamedPipe => FileTypeSimple::NamedPipe,
            FileType::CharDevice => FileTypeSimple::CharDevice,
            FileType::BlockDevice => FileTypeSimple::BlockDevice,
            FileType::Directory => FileTypeSimple::Directory,
            FileType::Symlink => FileTypeSimple::Symlink,
            FileType::Socket => FileTypeSimple::Socket,
        }
    }
}

impl TryFrom<u32> for FileTypeSimple {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FileTypeSimple::RegularFile),
            1 => Ok(FileTypeSimple::NamedPipe),
            2 => Ok(FileTypeSimple::CharDevice),
            3 => Ok(FileTypeSimple::BlockDevice),
            4 => Ok(FileTypeSimple::Directory),
            5 => Ok(FileTypeSimple::Symlink),
            6 => Ok(FileTypeSimple::Socket),
            _ => Err(format!("Unkown value: {}", value)),
        }
    }
}

impl From<FileTypeSimple> for u32 {
    fn from(value: FileTypeSimple) -> Self {
        match value {
            FileTypeSimple::RegularFile => 0,
            FileTypeSimple::NamedPipe => 1,
            FileTypeSimple::CharDevice => 2,
            FileTypeSimple::BlockDevice => 3,
            FileTypeSimple::Directory => 4,
            FileTypeSimple::Symlink => 5,
            FileTypeSimple::Socket => 6,
        }
    }
}

impl TryFrom<u8> for FileTypeSimple {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FileTypeSimple::RegularFile),
            1 => Ok(FileTypeSimple::NamedPipe),
            2 => Ok(FileTypeSimple::CharDevice),
            3 => Ok(FileTypeSimple::BlockDevice),
            4 => Ok(FileTypeSimple::Directory),
            5 => Ok(FileTypeSimple::Symlink),
            6 => Ok(FileTypeSimple::Socket),
            _ => Err(format!("Unkown value: {}", value)),
        }
    }
}

impl From<FileTypeSimple> for u8 {
    fn from(value: FileTypeSimple) -> Self {
        match value {
            FileTypeSimple::RegularFile => 0,
            FileTypeSimple::NamedPipe => 1,
            FileTypeSimple::CharDevice => 2,
            FileTypeSimple::BlockDevice => 3,
            FileTypeSimple::Directory => 4,
            FileTypeSimple::Symlink => 5,
            FileTypeSimple::Socket => 6,
        }
    }
}

pub fn file_attr_as_bytes(attr: &FileAttr) -> &[u8] {
    unsafe {
        let ptr = attr as *const FileAttr as *const u8;
        std::slice::from_raw_parts(ptr, std::mem::size_of::<FileAttr>())
    }
}

pub fn file_attr_as_bytes_mut(attr: &mut FileAttr) -> &mut [u8] {
    unsafe {
        let ptr = attr as *mut FileAttr as *mut u8;
        std::slice::from_raw_parts_mut(ptr, std::mem::size_of::<FileAttr>())
    }
}

pub fn bytes_as_file_attr(bytes: &[u8]) -> &FileAttr {
    unsafe {
        let ptr = bytes.as_ptr() as *const FileAttr;
        &*ptr
    }
}

pub fn bytes_as_file_attr_mut(bytes: &mut [u8]) -> &mut FileAttr {
    unsafe {
        let ptr = bytes.as_mut_ptr() as *mut FileAttr;
        &mut *ptr
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct FileAttrSimple {
    pub size: u64,
    pub blocks: u64,
    pub atime: SystemTime,
    pub mtime: SystemTime,
    pub ctime: SystemTime,
    pub crtime: SystemTime,
    pub kind: u32,
    pub perm: u16,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub rdev: u32,
    pub flags: u32,
    pub blksize: u32,
}

impl Default for FileAttrSimple {
    fn default() -> Self {
        Self::new(FileTypeSimple::RegularFile)
    }
}

impl FileAttrSimple {
    pub fn new(r#type: FileTypeSimple) -> Self {
        let kind = match r#type {
            FileTypeSimple::NamedPipe => 0,
            FileTypeSimple::CharDevice => 1,
            FileTypeSimple::BlockDevice => 2,
            FileTypeSimple::Directory => 3,
            FileTypeSimple::RegularFile => 4,
            FileTypeSimple::Symlink => 5,
            FileTypeSimple::Socket => 6,
        };
        let size = match r#type {
            FileTypeSimple::Directory => 4096,
            _ => 0,
        };
        FileAttrSimple {
            size,
            blocks: 0,
            atime: SystemTime::now(),
            mtime: SystemTime::now(),
            ctime: SystemTime::now(),
            crtime: SystemTime::now(),
            kind,
            perm: 0,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0,
            blksize: 0,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const FileAttrSimple as *const u8,
                std::mem::size_of::<FileAttrSimple>(),
            )
        }
    }

    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self as *mut FileAttrSimple as *mut u8,
                std::mem::size_of::<FileAttrSimple>(),
            )
        }
    }

    // pub fn to_bytes(attr: &FileAttrSimple) -> Vec<u8> {
    //     let mut bytes = Vec::with_capacity(8 + 8 + 8 + 8 + 8 + 8 + 4 + 2 + 4 + 4 + 4 + 4 + 4 + 4);
    //     bytes.extend_from_slice(&attr.size.to_le_bytes());
    //     bytes.extend_from_slice(&attr.blocks.to_le_bytes());
    //     bytes.extend_from_slice(&attr.atime.duration_since(UNIX_EPOCH).unwrap().as_secs().to_le_bytes());
    //     bytes.extend_from_slice(&attr.mtime.duration_since(UNIX_EPOCH).unwrap().as_secs().to_le_bytes());
    //     bytes.extend_from_slice(&attr.ctime.duration_since(UNIX_EPOCH).unwrap().as_secs().to_le_bytes());
    //     bytes.extend_from_slice(&attr.crtime.duration_since(UNIX_EPOCH).unwrap().as_secs().to_le_bytes());
    //     bytes.extend_from_slice(&attr.kind.to_le_bytes());
    //     bytes.extend_from_slice(&attr.perm.to_le_bytes());
    //     bytes.extend_from_slice(&attr.nlink.to_le_bytes());
    //     bytes.extend_from_slice(&attr.uid.to_le_bytes());
    //     bytes.extend_from_slice(&attr.gid.to_le_bytes());
    //     bytes.extend_from_slice(&attr.rdev.to_le_bytes());
    //     bytes.extend_from_slice(&attr.flags.to_le_bytes());
    //     bytes.extend_from_slice(&attr.blksize.to_le_bytes());
    //     bytes
    // }

    // pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
    //     if bytes.len() != 8 + 8 + 8 + 8 + 8 + 8 + 4 + 2 + 4 + 4 + 4 + 4 + 4 + 4 {
    //         return Err(format!("Invalid length: {}", bytes.len()));
    //     }
    //     let mut attr = FileAttrSimple::default();
    //     attr.size = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
    //     attr.blocks = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
    //     attr.atime = UNIX_EPOCH + Duration::from_secs(u64::from_le_bytes(bytes[16..24].try_into().unwrap()));
    //     attr.mtime = UNIX_EPOCH + Duration::from_secs(u64::from_le_bytes(bytes[24..32].try_into().unwrap()));
    //     attr.ctime = UNIX_EPOCH + Duration::from_secs(u64::from_le_bytes(bytes[32..40].try_into().unwrap()));
    //     attr.crtime = UNIX_EPOCH + Duration::from_secs(u64::from_le_bytes(bytes[40..48].try_into().unwrap()));
    //     attr.kind = u32::from_le_bytes(bytes[48..52].try_into().unwrap());
    //     attr.perm = u16::from_le_bytes(bytes[52..54].try_into().unwrap());
    //     attr.nlink = u32::from_le_bytes(bytes[54..58].try_into().unwrap());
    //     attr.uid = u32::from_le_bytes(bytes[58..62].try_into().unwrap());
    //     attr.gid = u32::from_le_bytes(bytes[62..66].try_into().unwrap());
    //     attr.rdev = u32::from_le_bytes(bytes[66..70].try_into().unwrap());
    //     attr.flags = u32::from_le_bytes(bytes[70..74].try_into().unwrap());
    //     attr.blksize = u32::from_le_bytes(bytes[74..78].try_into().unwrap());
    //     Ok(attr)
    // }
}
pub fn tostat(attr: &FileAttr, statbuf: &mut [u8]) {
    let kind = match attr.kind {
        FileType::NamedPipe => S_IFIFO,
        FileType::CharDevice => S_IFCHR,
        FileType::BlockDevice => S_IFBLK,
        FileType::Directory => S_IFDIR,
        FileType::RegularFile => S_IFREG,
        FileType::Symlink => S_IFLNK,
        FileType::Socket => S_IFSOCK,
    };
    unsafe {
        (*(statbuf.as_mut_ptr() as *mut stat)).st_dev = 0;
        (*(statbuf.as_mut_ptr() as *mut stat)).st_ino = 0;
        (*(statbuf.as_mut_ptr() as *mut stat)).st_mode = kind | attr.perm as u32;
        (*(statbuf.as_mut_ptr() as *mut stat)).st_nlink = attr.nlink as u64;
        (*(statbuf.as_mut_ptr() as *mut stat)).st_uid = attr.uid;
        (*(statbuf.as_mut_ptr() as *mut stat)).st_gid = attr.gid;
        (*(statbuf.as_mut_ptr() as *mut stat)).st_rdev = attr.rdev as u64;
        (*(statbuf.as_mut_ptr() as *mut stat)).st_size = attr.size as i64;
        (*(statbuf.as_mut_ptr() as *mut stat)).st_blksize = attr.blksize as i64;
        (*(statbuf.as_mut_ptr() as *mut stat)).st_blocks = attr.blocks as i64;
        (*(statbuf.as_mut_ptr() as *mut stat)).st_atime =
            attr.atime.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        (*(statbuf.as_mut_ptr() as *mut stat)).st_atime_nsec =
            attr.atime.duration_since(UNIX_EPOCH).unwrap().as_nanos() as i64;
        (*(statbuf.as_mut_ptr() as *mut stat)).st_mtime =
            attr.mtime.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        (*(statbuf.as_mut_ptr() as *mut stat)).st_mtime_nsec =
            attr.mtime.duration_since(UNIX_EPOCH).unwrap().as_nanos() as i64;
        (*(statbuf.as_mut_ptr() as *mut stat)).st_ctime =
            attr.ctime.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        (*(statbuf.as_mut_ptr() as *mut stat)).st_ctime_nsec =
            attr.ctime.duration_since(UNIX_EPOCH).unwrap().as_nanos() as i64;
    }
}
pub fn tostatx(attr: &FileAttr, statxbuf: &mut [u8]) {
    let kind = match attr.kind {
        FileType::NamedPipe => S_IFIFO,
        FileType::CharDevice => S_IFCHR,
        FileType::BlockDevice => S_IFBLK,
        FileType::Directory => S_IFDIR,
        FileType::RegularFile => S_IFREG,
        FileType::Symlink => S_IFLNK,
        FileType::Socket => S_IFSOCK,
    } as u16;

    unsafe {
        (*(statxbuf.as_mut_ptr() as *mut statx)).stx_mask = 0;
        (*(statxbuf.as_mut_ptr() as *mut statx)).stx_ino = 0;
        (*(statxbuf.as_mut_ptr() as *mut statx)).stx_mode = kind | attr.perm;
        (*(statxbuf.as_mut_ptr() as *mut statx)).stx_nlink = attr.nlink;
        (*(statxbuf.as_mut_ptr() as *mut statx)).stx_uid = attr.uid;
        (*(statxbuf.as_mut_ptr() as *mut statx)).stx_gid = attr.gid;
        (*(statxbuf.as_mut_ptr() as *mut statx)).stx_size = attr.size;
        (*(statxbuf.as_mut_ptr() as *mut statx)).stx_blksize = attr.blksize;
        (*(statxbuf.as_mut_ptr() as *mut statx)).stx_blocks = attr.blocks;
        (*(statxbuf.as_mut_ptr() as *mut statx)).stx_atime = statx_timestamp {
            tv_sec: attr.atime.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            tv_nsec: 0,
            __statx_timestamp_pad1: [0i32; 1],
        };
        (*(statxbuf.as_mut_ptr() as *mut statx)).stx_btime = statx_timestamp {
            tv_sec: 0,
            tv_nsec: 0,
            __statx_timestamp_pad1: [0i32; 1],
        };
        (*(statxbuf.as_mut_ptr() as *mut statx)).stx_mtime = statx_timestamp {
            tv_sec: attr.mtime.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            tv_nsec: 0,
            __statx_timestamp_pad1: [0i32; 1],
        };
        (*(statxbuf.as_mut_ptr() as *mut statx)).stx_ctime = statx_timestamp {
            tv_sec: attr.ctime.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            tv_nsec: 0,
            __statx_timestamp_pad1: [0i32; 1],
        };
    }
}

impl From<FileAttrSimple> for fuser::FileAttr {
    fn from(attr: FileAttrSimple) -> Self {
        let kind = match attr.kind {
            0 => FileType::NamedPipe,
            1 => FileType::CharDevice,
            2 => FileType::BlockDevice,
            3 => FileType::Directory,
            4 => FileType::RegularFile,
            5 => FileType::Symlink,
            6 => FileType::Socket,
            _ => FileType::RegularFile,
        };
        fuser::FileAttr {
            ino: 0,
            size: attr.size,
            blocks: attr.blocks,
            atime: attr.atime,
            mtime: attr.mtime,
            ctime: attr.ctime,
            crtime: attr.crtime,
            kind,
            perm: attr.perm,
            nlink: attr.nlink,
            uid: attr.uid,
            gid: attr.gid,
            rdev: attr.rdev,
            flags: attr.flags,
            blksize: attr.blksize,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SubDirectory {
    pub sub_dir: BTreeMap<String, String>,
}

impl Default for SubDirectory {
    fn default() -> Self {
        Self::new()
    }
}

impl SubDirectory {
    pub fn new() -> Self {
        let sub_dir = BTreeMap::from([
            (".".to_string(), "d".to_string()),
            ("..".to_string(), "d".to_string()),
        ]);
        SubDirectory { sub_dir }
    }

    pub fn add_dir(&mut self, dir: String) {
        self.sub_dir.insert(dir, "d".to_string());
    }

    pub fn add_file(&mut self, file: String) {
        self.sub_dir.insert(file, "f".to_string());
    }

    pub fn delete_dir(&mut self, dir: String) {
        self.sub_dir.remove(&dir);
    }

    pub fn delete_file(&mut self, file: String) {
        self.sub_dir.remove(&file);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ReadFileSendMetaData {
    pub offset: i64,
    pub size: u32,
}

#[repr(C)]
pub struct LinuxDirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_name: [i8; 256],
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct WriteFileSendMetaData {
    pub offset: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DirectoryEntrySendMetaData {
    pub file_type: u8,
    pub file_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TruncateFileSendMetaData {
    pub length: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ReadDirSendMetaData {
    pub offset: i64,
    pub size: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct OpenFileSendMetaData {
    pub flags: i32,
    pub mode: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CreateFileSendMetaData {
    pub mode: u32,
    pub umask: u32,
    pub flags: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DeleteFileSendMetaData {
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CreateDirSendMetaData {
    pub mode: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DeleteDirSendMetaData {
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct UpdateServerStatusSendMetaData {
    pub status: ServerStatus,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct GetClusterStatusRecvMetaData {
    pub status: ClusterStatus,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct GetHashRingInfoRecvMetaData {
    pub hash_ring_info: Vec<(String, usize)>,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct AddNodesSendMetaData {
    pub new_servers_info: Vec<(String, usize)>,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct DeleteNodesSendMetaData {
    pub deleted_servers_info: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct CheckFileSendMetaData {
    pub file_attr: FileAttrSimple,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct CheckDirSendMetaData {
    pub file_attr: FileAttrSimple,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct CreateVolumeSendMetaData {
    pub size: u64,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct MountVolumeSendMetaData {
    pub volume_name: String,
    pub mount_point: String,
    pub read_only: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Volume {
    pub name: String,
    pub size: u64,
    pub used_size: u64,
}

impl Display for Volume {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Volume {{ name: {}, size: {}, used_size: {} }}",
            self.name, self.size, self.used_size
        )
    }
}

use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[repr(u8)] pub enum Instruction {
	PerformScript = 1,
	SaveACopyAsXml = 3,
	GoToNextField = 4,
	GoToPreviousField = 5,
	GoToLayout = 6,
	NewRecordRequest = 7,
    DuplicateRecordRequest = 8,
	DeleteRecordRequest = 9,
	DeleteAllRecords = 10,
	InsertFromIndex = 11,
	InsertFromLastVisited = 12,
	InsertCurrentDate = 13,
	InsertCurrentTime = 14,
	GoToRecordRequestPage = 16,
	GoToField = 17,
	CheckSelection = 18,
	CheckRecord = 19,
	CheckFoundSet = 20,
	UnsortRecords = 21,
	EnterFindMode = 22,
	ShowAllRecords = 23,
	ModifyLastFind = 24,
	OmitRecord = 25,
	OmitMultipleRecords = 26,
	ShowOmmitedOnly = 27,
	PerformFind = 28,
	ShowHideToolbars = 29,
	ViewAs = 30,
	AdjustWindow = 31,
	OpenHelp = 32,
	OpenFile = 33,
	CloseFile = 34,
	ImportRecords = 35,
	ExportRecords = 36,
	SaveACopyAs = 37,
	OpenManageDatabase = 38,
	SortRecords = 39,
	RelookupFieldContents = 40,
	EnterPreviewMode = 41,
	PrintSetup = 42,
	Print = 43,
	ExitApplication = 44,
	UndoRedo = 45,
	Cut = 46,
	Copy = 47,
	Paste = 48,
	Clear = 49,
	SelectAll = 50,
	RevertRecordRequest = 51,
	EnterBrowserMode = 55,
	InsertPicture = 56,
	SendEvent = 57,
	InsertCurrentUserName = 60,
	InsertText = 61,
	PauseResumeScript = 62,
	SendMail = 63,
	SendDdeExecute = 64,
	DialPhone = 65,
	Speak = 66,
	PerformApplescript = 67,
	If = 68,
	Else = 69,
	EndIf = 70,
	Loop = 71,
	ExitLoopIf = 72,
	EndLoop = 73,
	GoToRelatedRecord = 74,
	CommitRecordsRequests = 75,
	SetField = 76,
	InsertCalculatedResult = 77,
	FreezeWindow = 79,
	RefreshWindow = 80,
	ScrollWindow = 81,
	NewFile = 82,
	ChangePassword = 83,
	SetMultiUser = 84,
	AllowUserAbort = 85,
	SetErrorCapture = 86,
	ShowCustomDialog = 87,
	OpenScriptWorkspace = 88,
	BlankLineComment = 89,
	HaltScript = 90,
	ReplaceFieldContents = 91,
	ShowHideTextRuler = 92,
	Beep = 93,
	SetUseSystemFormats = 94,
	RecoverFile = 95,
	SaveACopyAsAddOnPackage = 96,
	SetZoomLevel = 97,
	CopyAllRecordsRequests = 98,
	GoToPortalRow = 99,
	CopyRecordRequest = 101,
	FluchCacheToDisk = 102,
	ExitScript = 103,
	DeletePortalRow = 104,
	OpenPreferences = 105,
	CorrectWord = 106,
	SpellingOptions = 107,
	SelectDictionaries = 108,
	EditUserDictionary = 109,
	OpenUrl = 111,
	OpenManageValueLists = 112,
	OpenSharing = 113,
	OpenFileOptions = 114,
	AllowFormattingBar = 115,
	SetNextSerialValue = 116,
	ExecuteSql = 117,
	OpenHosts = 118,
	MoveResizeWindow = 119,
	ArrangeAllWindows = 120,
	CloseWindow = 121,
	NewWindow = 122,
	SelectWindow = 123,
	SetWindowTitle = 124,
	ElseIf = 125,
	ConstrainFoundSet = 126,
	ExtendFoundSet = 127,
	PerformFindReplace = 128,
	OpenFindReplace = 129,
	SetSelection = 130,
	InsertFile = 131,
	ExportFieldContents = 132,
	OpenRecordRequest = 133,
	AddAccount = 134,
	DeleteAccount = 135,
	ResetAccountPassword = 136,
	EnableAccount = 137,
	Relogin = 138,
	ConvertFile = 139,
	OpenManageDataSources = 140,
	SetVariable = 141,
	InstallMenuSet = 142,
	SaveRecordsAsExcel = 143,
	SaveRecordsAsPdf = 144,
	GoToObject = 145,
	SetWebViewer = 146,
	SetFieldByName = 147,
	InstallOntimerScript = 148,
	OpenEditSavedFinds = 149,
	PerformQuickFind = 150,
	OpenManageLayouts = 151,
	SaveRecordsAsSnapshotLink = 152,
	SortRecordsByField = 154,
	FindMatchingRecords = 155,
	ManageContainers = 156,
	InstallPluginFile = 157,
	InsertPdf = 158,
	InsertAudioVideo = 159,
	InsertFromUrl = 160,
	InsertFromDevice = 161,
	PerformScriptOnServer = 164,
	OpenManageThemes = 165,
	ShowHideMenubar = 166,
	RefreshObject = 167,
	SetLayoutObjectAnimation = 168,
	ClosePopover = 169,
	OpenUploadToHost = 172,
	EnableTouchKeyboard = 174,
	PerformJavascriptInWebViewer = 175,
    CommentedOut = 176,
	AvplayerPlay = 177,
	AvplayerSetPlaybackState = 178,
	AvplayerSetOptions = 179,
	RefreshPortal = 180,
	GetFolderPath = 181,
	TruncateTable = 182,
	OpenFavorites = 183,
	ConfigureRegionMonitorScript = 185,
	ConfigureLocalNotification = 187,
	GetFileExists = 188,
	GetFileSize = 189,
	CreateDataFile = 190,
	OpenDataFile = 191,
	WriteToDataFile = 192,
	ReadFromDataFile = 193,
	GetDataFilePosition = 194,
	SetDataFilePosition = 195,
	CloseDataFile = 196,
	DeleteFile = 197,
	RenameFile = 199,
	SetErrorLogging = 200,
	ConfigureNfcReading = 201,
	ConfigureMachineLearningModel = 202,
	ExecuteFilemakerDataApi = 203,
	OpenTransaction = 205,
	CommitTransaction = 206,
	RevertTransaction = 207,
	SetSessionIdentifier = 208,
	SetDictionary = 209,
	PerformScriptOnServerWithCallback = 210,
	TriggerClarisConnectFlow = 211,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Instruction, Self::Err> {
        match input {
            "perform_script" => Ok(Instruction::PerformScript),
            "set_variable" => Ok(Instruction::SetVariable),
            "loop" => Ok(Instruction::Loop),
            "exit_loop_if" => Ok(Instruction::ExitLoopIf),
            "exit_script" => Ok(Instruction::ExitScript),
            _ => Err(()),
        }
    }

}

pub static INSTRUCTIONMAP : [Option<Instruction>; 212] = [
    None,
	Some(Instruction::PerformScript),
    None,
	Some(Instruction::SaveACopyAsXml),
	Some(Instruction::GoToNextField),
	Some(Instruction::GoToPreviousField),
	Some(Instruction::GoToLayout),
	Some(Instruction::NewRecordRequest),
    Some(Instruction::DuplicateRecordRequest),
	Some(Instruction::DeleteRecordRequest),
	Some(Instruction::DeleteAllRecords),
	Some(Instruction::InsertFromIndex),
	Some(Instruction::InsertFromLastVisited),
	Some(Instruction::InsertCurrentDate),
	Some(Instruction::InsertCurrentTime),
    None,
	Some(Instruction::GoToRecordRequestPage),
	Some(Instruction::GoToField),
	Some(Instruction::CheckSelection),
	Some(Instruction::CheckRecord),
	Some(Instruction::CheckFoundSet),
	Some(Instruction::UnsortRecords),
	Some(Instruction::EnterFindMode),
	Some(Instruction::ShowAllRecords),
	Some(Instruction::ModifyLastFind),
	Some(Instruction::OmitRecord),
	Some(Instruction::OmitMultipleRecords),
	Some(Instruction::ShowOmmitedOnly),
	Some(Instruction::PerformFind),
	Some(Instruction::ShowHideToolbars),
	Some(Instruction::ViewAs),
	Some(Instruction::AdjustWindow),
	Some(Instruction::OpenHelp),
	Some(Instruction::OpenFile),
	Some(Instruction::CloseFile),
	Some(Instruction::ImportRecords),
	Some(Instruction::ExportRecords),
	Some(Instruction::SaveACopyAs),
	Some(Instruction::OpenManageDatabase),
	Some(Instruction::SortRecords),
	Some(Instruction::RelookupFieldContents),
	Some(Instruction::EnterPreviewMode),
	Some(Instruction::PrintSetup),
	Some(Instruction::Print),
	Some(Instruction::ExitApplication),
	Some(Instruction::UndoRedo),
	Some(Instruction::Cut),
	Some(Instruction::Copy),
	Some(Instruction::Paste),
	Some(Instruction::Clear),
	Some(Instruction::SelectAll),
	Some(Instruction::RevertRecordRequest),
    None,
    None,
    None,
	Some(Instruction::EnterBrowserMode),
	Some(Instruction::InsertPicture),
	Some(Instruction::SendEvent),
    None,
    None,
	Some(Instruction::InsertCurrentUserName),
	Some(Instruction::InsertText),
	Some(Instruction::PauseResumeScript),
	Some(Instruction::SendMail),
	Some(Instruction::SendDdeExecute),
	Some(Instruction::DialPhone),
	Some(Instruction::Speak),
	Some(Instruction::PerformApplescript),
	Some(Instruction::If),
	Some(Instruction::Else),
	Some(Instruction::EndIf),
	Some(Instruction::Loop),
	Some(Instruction::ExitLoopIf),
	Some(Instruction::EndLoop),
	Some(Instruction::GoToRelatedRecord),
	Some(Instruction::CommitRecordsRequests),
	Some(Instruction::SetField),
	Some(Instruction::InsertCalculatedResult),
    None,
	Some(Instruction::FreezeWindow),
	Some(Instruction::RefreshWindow),
	Some(Instruction::ScrollWindow),
	Some(Instruction::NewFile),
	Some(Instruction::ChangePassword),
	Some(Instruction::SetMultiUser),
	Some(Instruction::AllowUserAbort),
	Some(Instruction::SetErrorCapture),
	Some(Instruction::ShowCustomDialog),
	Some(Instruction::OpenScriptWorkspace),
	Some(Instruction::BlankLineComment),
	Some(Instruction::HaltScript),
	Some(Instruction::ReplaceFieldContents),
	Some(Instruction::ShowHideTextRuler),
	Some(Instruction::Beep),
	Some(Instruction::SetUseSystemFormats),
	Some(Instruction::RecoverFile),
	Some(Instruction::SaveACopyAsAddOnPackage),
	Some(Instruction::SetZoomLevel),
	Some(Instruction::CopyAllRecordsRequests),
	Some(Instruction::GoToPortalRow),
    None,
	Some(Instruction::CopyRecordRequest),
	Some(Instruction::FluchCacheToDisk),
	Some(Instruction::ExitScript),
	Some(Instruction::DeletePortalRow),
	Some(Instruction::OpenPreferences),
	Some(Instruction::CorrectWord),
	Some(Instruction::SpellingOptions),
	Some(Instruction::SelectDictionaries),
	Some(Instruction::EditUserDictionary),
    None,
	Some(Instruction::OpenUrl),
	Some(Instruction::OpenManageValueLists),
	Some(Instruction::OpenSharing),
	Some(Instruction::OpenFileOptions),
	Some(Instruction::AllowFormattingBar),
	Some(Instruction::SetNextSerialValue),
	Some(Instruction::ExecuteSql),
	Some(Instruction::OpenHosts),
	Some(Instruction::MoveResizeWindow),
	Some(Instruction::ArrangeAllWindows),
	Some(Instruction::CloseWindow),
	Some(Instruction::NewWindow),
	Some(Instruction::SelectWindow),
	Some(Instruction::SetWindowTitle),
	Some(Instruction::ElseIf),
	Some(Instruction::ConstrainFoundSet),
	Some(Instruction::ExtendFoundSet),
	Some(Instruction::PerformFindReplace),
	Some(Instruction::OpenFindReplace),
	Some(Instruction::SetSelection),
	Some(Instruction::InsertFile),
	Some(Instruction::ExportFieldContents),
	Some(Instruction::OpenRecordRequest),
	Some(Instruction::AddAccount),
	Some(Instruction::DeleteAccount),
	Some(Instruction::ResetAccountPassword),
	Some(Instruction::EnableAccount),
	Some(Instruction::Relogin),
	Some(Instruction::ConvertFile),
	Some(Instruction::OpenManageDataSources),
	Some(Instruction::SetVariable),
	Some(Instruction::InstallMenuSet),
	Some(Instruction::SaveRecordsAsExcel),
	Some(Instruction::SaveRecordsAsPdf),
	Some(Instruction::GoToObject),
	Some(Instruction::SetWebViewer),
	Some(Instruction::SetFieldByName),
	Some(Instruction::InstallOntimerScript),
	Some(Instruction::OpenEditSavedFinds),
	Some(Instruction::PerformQuickFind),
	Some(Instruction::OpenManageLayouts),
	Some(Instruction::SaveRecordsAsSnapshotLink),
    None,
	Some(Instruction::SortRecordsByField),
	Some(Instruction::FindMatchingRecords),
	Some(Instruction::ManageContainers),
	Some(Instruction::InstallPluginFile),
	Some(Instruction::InsertPdf),
	Some(Instruction::InsertAudioVideo),
	Some(Instruction::InsertFromUrl),
	Some(Instruction::InsertFromDevice),
    None,
    None,
	Some(Instruction::PerformScriptOnServer),
	Some(Instruction::OpenManageThemes),
	Some(Instruction::ShowHideMenubar),
	Some(Instruction::RefreshObject),
	Some(Instruction::SetLayoutObjectAnimation),
	Some(Instruction::ClosePopover),
    None,
    None,
	Some(Instruction::OpenUploadToHost),
    None,
	Some(Instruction::EnableTouchKeyboard),
	Some(Instruction::PerformJavascriptInWebViewer),
    Some(Instruction::CommentedOut),
	Some(Instruction::AvplayerPlay),
	Some(Instruction::AvplayerSetPlaybackState),
	Some(Instruction::AvplayerSetOptions),
	Some(Instruction::RefreshPortal),
	Some(Instruction::GetFolderPath),
	Some(Instruction::TruncateTable),
	Some(Instruction::OpenFavorites),
    None,
	Some(Instruction::ConfigureRegionMonitorScript),
    None,
	Some(Instruction::ConfigureLocalNotification),
	Some(Instruction::GetFileExists),
	Some(Instruction::GetFileSize),
	Some(Instruction::CreateDataFile),
	Some(Instruction::OpenDataFile),
	Some(Instruction::WriteToDataFile),
	Some(Instruction::ReadFromDataFile),
	Some(Instruction::GetDataFilePosition),
	Some(Instruction::SetDataFilePosition),
	Some(Instruction::CloseDataFile),
	Some(Instruction::DeleteFile),
    None,
	Some(Instruction::RenameFile),
	Some(Instruction::SetErrorLogging),
	Some(Instruction::ConfigureNfcReading),
	Some(Instruction::ConfigureMachineLearningModel),
	Some(Instruction::ExecuteFilemakerDataApi),
    None,
	Some(Instruction::OpenTransaction),
	Some(Instruction::CommitTransaction),
	Some(Instruction::RevertTransaction),
	Some(Instruction::SetSessionIdentifier),
	Some(Instruction::SetDictionary),
	Some(Instruction::PerformScriptOnServerWithCallback),
	Some(Instruction::TriggerClarisConnectFlow),
];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScriptStep {
    pub opcode: Instruction,
    pub index: usize,
    pub switches: Vec<String>,
}

pub struct Script {
    pub script_name: String,
    pub instructions: Vec<Instruction>,
}


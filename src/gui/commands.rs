pub mod blob_commands
{
    use druid::Selector;

    /// Shows the crop window menu
    pub const SHOW_CROP: Selector = Selector::new("blob-command.show-crop");

    /// Shows the rotate window menu
    pub const SHOW_ROTATE: Selector = Selector::new("blob-command.show-rotate");
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Category {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateCategoryCommand {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateCategoryCommand {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteCategoryCommand {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CategorySearchQuery {}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CategoryQuery {
    pub id: String,
}

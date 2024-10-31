enum Page {
    ChooseRoute,
    ChooseTrain,
    Waiting
}

struct User {
    chat_id: i64,
    page: Page,

}
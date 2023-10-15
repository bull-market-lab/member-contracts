use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint64;

#[cw_serde]
pub struct Thread {
    // Thread ID, a global unique identifier that is monotonically increasing
    pub id: Uint64,
    // Thread title
    // TODO: P0: decide if we should use base64 encode so we can display more languages, this has no impact on contract, it's mainly a frontend thing
    pub title: String,
    // Thread description
    pub description: String,
    // List of labels
    // pub labels: Vec<String>,
    // TODO: P1: introducing secondary label? e.g. label: "cosmwasm" and secondary_label: "warp"
    // Thread creator's user ID in membership contract
    pub creator_user_id: Uint64,
    // Whether this thread and msg under it is updatable
    pub updatable: bool,
    // Whether this thread and msg under it is deletable
    pub deletable: bool,
    // TODO: add thread level fee config
}

#[cw_serde]
pub enum ThreadMsg {
    // Membership holder can pay to ask a question to key issuer
    ThreadQuestionMsg(ThreadQuestionMsg),
    // Membership issuer can answer the question asked by key holder and collect the reward
    ThreadAnswerMsg(ThreadAnswerMsg),
    // As long as user holds the thread creator's key, user can reply to the thread
    // If user wants to reply to an answer or a question or a reply
    // User must hold the key of the person who answered or asked or replied
    ThreadReplyMsg(ThreadReplyMsg),
}

#[cw_serde]
pub struct ThreadQuestionMsg {
    // Thread msg ID, a thread level unique identifier that is monotonically increasing
    pub id: Uint64,
    // ID of the thread that this question belongs to
    pub thread_id: Uint64,
    // Question creator's user ID in membership contract
    pub creator_user_id: Uint64,
    // Question content
    pub content: String,
    // Each question must be asked to a specific user, ask to user's user ID in membership contract
    pub asked_to_user_id: Uint64,
    // TODO: P0: support adding tip in case membership price too low
    // Tip will be split between members and issuer, but excluding protocol fee collector
}

#[cw_serde]
pub struct ThreadAnswerMsg {
    // Thread msg ID, a thread level unique identifier that is monotonically increasing
    pub id: Uint64,
    // ID of the thread that this answer belongs to
    pub thread_id: Uint64,
    // Answer creator's user ID in membership contract
    pub creator_user_id: Uint64,
    // Answer content
    pub content: String,
    // Each answer must be answering to a specific question ID
    // A question can have multiple answers
    pub answered_to_question_msg_id: Uint64,
}

#[cw_serde]
pub struct ThreadReplyMsg {
    // Thread msg ID, a thread level unique identifier that is monotonically increasing
    pub id: Uint64,
    // ID of the thread that this reply belongs to
    pub thread_id: Uint64,
    // Reply creator's user ID in membership contract
    pub creator_user_id: Uint64,
    // Reply content
    pub content: String,
    // A reply can be a reply in the thread or a reply to another reply in the thread
    // Or a reply to an answer or a question in the thread
    // reply_to_msg_id will be empty if this reply is a reply to the thread
    pub reply_to_thread_msg_id: Option<Uint64>,
}

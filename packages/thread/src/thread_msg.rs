use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};

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
    // Address of the person who asked the question
    pub creator_addr: Addr,
    // Question content
    pub content: String,
    // Each question must be asked to a specific user
    pub asked_to_addr: Addr,
}

#[cw_serde]
pub struct ThreadAnswerMsg {
    // Thread msg ID, a thread level unique identifier that is monotonically increasing
    pub id: Uint64,
    // ID of the thread that this answer belongs to
    pub thread_id: Uint64,
    // Address of the person who answered the question
    // At this moment it's always the key issuer of the thread that this answer belongs to
    pub creator_addr: Addr,
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
    // Address of the person who replied to the thread or a reply or an answer or a question in the thread
    pub creator_addr: Addr,
    // Reply content
    pub content: String,
    // A reply can be a reply in the thread or a reply to another reply in the thread
    // Or a reply to an answer or a question in the thread
    // reply_to_msg_id will be empty if this reply is a reply to the thread
    pub reply_to_thread_msg_id: Option<Uint64>,
}

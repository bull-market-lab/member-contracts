use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};

#[cw_serde]
pub enum ThreadMsg {
    // Key holder can pay to ask a question to key issuer
    Question(Question),
    // Key issuer can answer the question asked by key holder and collect the reward
    Answer(Answer),
    /*
    TODO: add more types of thread msg
        such as comment (not sure if it's a good idea to enable free comment) so key holders can comment on each other
     */
}

#[cw_serde]
pub struct Question {
    // Thread msg ID, a thread level unique identifier that is monotonically increasing
    pub id: Uint64,
    // ID of the thread that this question belongs to
    pub thread_id: Uint64,
    // Address of the person who asked the question
    pub ask_by_addr: Addr,
    // Question content
    pub content: String,
}

#[cw_serde]
pub struct Answer {
    // Thread msg ID, a thread level unique identifier that is monotonically increasing
    pub id: Uint64,
    // ID of the thread that this answer belongs to
    pub thread_id: Uint64,
    // Address of the person who answered the question
    // At this moment it's always the key issuer of the thread that this answer belongs to
    pub answer_by_addr: Addr,
    // Answer content
    pub content: String,
    // Each answer must be replying to a specific question ID
    // A question can have multiple answers
    pub reply_to_question_id: Uint64,
}

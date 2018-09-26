use Uint;

mod operation;

type UserId = Uint;

struct UserState {
    retained_token_count: Uint,
    total_tokens_inserted: Uint,
}


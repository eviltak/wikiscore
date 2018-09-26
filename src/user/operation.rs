use Uint;
use user::UserId;

/// Represents an operation performed in the future.
///
/// The first element is the number of revisions after which the operation is performed.
/// The second element is the operation performed.
type Future<O> = (Uint, O);

struct Insertion {
    retained_token_count: Uint,
    deleted_tokens: Vec<Future<Deletion>>,
}

struct Deletion {
    deleter: UserId,
}

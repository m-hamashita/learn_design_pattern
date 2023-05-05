pub struct UserCollection {
    users: [&'static str; 3],
}

// UserCollection に対して iter method を実装する
impl UserCollection {
    pub fn new() -> Self {
        Self {
            users: ["Alice", "Bob", "Carl"],
        }
    }

    // UserIterator を返す（Iterator trait を実装しているので Iterator として振る舞える）
    // iter method とするのが、慣習的によく使われる（デファクト）
    pub fn iter(&self) -> UserIterator {
        UserIterator {
            index: 0,
            user_collection: self,
        }
    }
}

pub struct UserIterator<'a> {
    index: usize,
    user_collection: &'a UserCollection,
}

impl Iterator for UserIterator<'_> {
    // Item は iterator が返す要素の型
    type Item = &'static str;

    // iterator trait は next method のみ実装する
    // index が len よりも小さい間は Some(T) を返し、index をインクリメントする
    // index が len に達したら None を返すようにする
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.user_collection.users.len() {
            let user = Some(self.user_collection.users[self.index]);
            self.index += 1;
            return user;
        }

        None
    }
}

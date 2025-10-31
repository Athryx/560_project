use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct User(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Group(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Resource(pub u64);

#[derive(Debug, Default)]
pub struct Context {
    // Direct user → resource allow rules
    user_rules: HashSet<(User, Resource)>,
    // Group → resource allow rules
    group_rules: HashSet<(Group, Resource)>,
    // Group memberships: group -> set of users
    group_memberships: HashMap<Group, HashSet<User>>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allow_user(&mut self, user: User, resource: Resource) {
        self.user_rules.insert((user, resource));
    }

    pub fn allow_group(&mut self, group: Group, resource: Resource) {
        self.group_rules.insert((group, resource));
    }

    pub fn add_user_to_group(&mut self, user: User, group: Group) {
        self.group_memberships
            .entry(group)
            .or_default()
            .insert(user);
    }

    pub fn is_authorized(&self, user: User, resource: Resource) -> bool {
        // 1. Direct rule for the user
        if self.user_rules.contains(&(user, resource)) {
            return true;
        }

        // 2. Check group rules for any group the user belongs to
        for (group, users) in &self.group_memberships {
            if users.contains(&user) && self.group_rules.contains(&(*group, resource)) {
                return true;
            }
        }

        false
    }
}

fn main() {
    let mut ctx = Context::new();

    let user1 = User(1);
    let user2 = User(2);
    let group_admins = Group(1);
    let resource_db = Resource(100);

    // Grant access to admins group
    ctx.allow_group(group_admins, resource_db);

    // Add user1 to admins
    ctx.add_user_to_group(user1, group_admins);

    // Directly allow user2
    ctx.allow_user(user2, resource_db);

    println!(
        "User1 authorized? {}",
        ctx.is_authorized(user1, resource_db)
    ); // true (via group)
    println!(
        "User2 authorized? {}",
        ctx.is_authorized(user2, resource_db)
    ); // true (direct rule)
    println!(
        "User3 authorized? {}",
        ctx.is_authorized(User(3), resource_db)
    ); // false
}

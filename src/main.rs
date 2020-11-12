use git2::{Repository, BranchType, Branch, Error, Oid};

fn main() {
    println!("Hello, world!");
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
    let main_branch = match repo.find_branch("origin/main", BranchType::Remote) {
        Ok(b) => b,
        Err(e) => panic!("failed to find main branch: {}", e),
    };

    let main_branch_oid = match main_branch.get().target() {
        None => panic!("failed to read oid of main branch"),
        Some(oid) => oid
    };
    let upstream_oid = Oid::zero();
    let (a,b) = match repo.graph_ahead_behind(main_branch_oid, upstream_oid){
        Ok(sizes) => sizes,
        Err(e) => panic!("failed to read sizes {}", e),
    };
    println!("{}", main_branch_oid);
     println!("{}, {}",a, b);
}

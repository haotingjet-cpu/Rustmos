#[derive(Debug, Clone, PartialEq)]
struct AstNode {
    state: NodeState,
    left: Option<Box<AstNode>>,
    right: Option<Box<AstNode>>,
    on_road: bool,
    level: i32,
}

#[derive(Debug, Clone, PartialEq)]
enum NodeState {
    Leaf(f32),
    Inter(Rule),
    Root,
}

#[derive(Debug, Clone, PartialEq)]
enum Rule {
    Add,
    Sub,
    Div,
    Mul,
}

struct Ast {
    root: AstNode,
}

impl Ast {
    fn new() -> Self {
        Self {
            root: AstNode {
                state: NodeState::Root,
                left: None,
                right: None,
                on_road: true,
                level: -1,
            },
        }
    }

    // 提供一個外部呼叫的 push 接口
    fn push(&mut self, node: AstNode) {
        self.root.push(node);
    }
}

impl AstNode {
    fn push(&mut self, mut node: AstNode) {
        // 使用 std::mem::replace 暫時把左子樹拿出來拿出來操作，避免使用 .clone()
        let mut left_opt = std::mem::replace(&mut self.left, None);

        match &mut left_opt {
            // ==================== 狀況一：左邊無東西 ====================
            None => {
                if node.level > self.level {
                    // 新節點優先級更高，直接作為左子樹，並保持在路徑上
                    node.on_road = true;
                    self.left = Some(Box::new(node));
                } else {
                    // 當前節點優先級更高：
                    // 1. 先把「當前節點」的舊狀態打包，放到左子樹（此時它離開路徑了，on_road = false）
                    self.left = Some(Box::new(AstNode {
                        state: std::mem::replace(&mut self.state, node.state),
                        left: None,
                        right: None,
                        on_road: false,
                        level: self.level,
                    }));
                    // 2. 當前節點被新節點取代
                    self.level = node.level;
                    self.on_road = true;
                }
            }

            // ==================== 狀況二：左邊已經有東西 ====================
            Some(left_node) => {
                if left_node.on_road {
                    // 子狀況 A：左邊仍在構建路徑上
                    if node.level > left_node.level {
                        // 新節點優先級更高，繼續往左子樹遞迴深入
                        left_node.push(node);
                        self.left = left_opt; // 把左子樹還回去
                    } else {
                        // 新節點優先級較低或相等：
                        // 在當前節點與左子節點之間「插入」一個新階層
                        let old_left_left = std::mem::replace(&mut left_node.left, None);
                        let old_left_right = std::mem::replace(&mut left_node.right, None);

                        // 把舊左節點的內容複製到一個新的子節點，並標記為完成 (on_road = false)
                        left_node.left = Some(Box::new(AstNode {
                            state: left_node.state.clone(),
                            left: old_left_left,
                            right: old_left_right,
                            on_road: false,
                            level: left_node.level,
                        }));

                        // 左子節點本身被新節點的內容覆蓋，並重置狀態
                        left_node.right = None;
                        left_node.level = node.level;
                        left_node.state = node.state;
                        left_node.on_road = true;

                        self.left = left_opt; // 把修改後的左子樹還回去
                    }
                } else {
                    // 子狀況 B：左邊已經完成 (on_road == false)
                    // 這時候需要跟「目前節點 (self)」比較優先級，而不是直接塞給右邊
                    self.left = left_opt; // 先把左子樹還回去

                    if node.level > self.level {
                        // 新節點比目前節點優先級高，應該走右邊
                        if let Some(right_node) = &mut self.right {
                            right_node.push(node);
                        } else {
                            node.on_road = true;
                            self.right = Some(Box::new(node));
                        }
                    } else {
                        // 新節點比目前節點優先級低（例如目前是 *，新節點是 +）：
                        // 目前整個節點（包含已完成的左右子樹）都要變成新節點的左子樹！
                        let old_state = std::mem::replace(&mut self.state, node.state);
                        let old_left = std::mem::replace(&mut self.left, None);
                        let old_right = std::mem::replace(&mut self.right, None);

                        self.left = Some(Box::new(AstNode {
                            state: old_state,
                            left: old_left,
                            right: old_right,
                            on_road: false,
                            level: self.level,
                        }));

                        self.level = node.level;
                        self.on_road = true;
                    }
                }
            }
        }
    }
}

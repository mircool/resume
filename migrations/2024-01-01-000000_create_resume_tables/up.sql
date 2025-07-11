-- 个人信息表
CREATE TABLE personal_info (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    phone VARCHAR,
    location VARCHAR,
    summary TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 技能表
CREATE TABLE skills (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    category VARCHAR NOT NULL, -- 技能分类：编程语言、框架、工具等
    proficiency INTEGER NOT NULL DEFAULT 1, -- 熟练度 1-5
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 工作经验表
CREATE TABLE work_experiences (
    id INTEGER PRIMARY KEY NOT NULL,
    company VARCHAR NOT NULL,
    position VARCHAR NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE,
    description TEXT,
    achievements TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 教育背景表
CREATE TABLE education (
    id INTEGER PRIMARY KEY NOT NULL,
    institution VARCHAR NOT NULL,
    degree VARCHAR NOT NULL,
    field_of_study VARCHAR,
    start_date DATE NOT NULL,
    end_date DATE,
    gpa VARCHAR,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 项目经验表
CREATE TABLE projects (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    description TEXT NOT NULL,
    technologies VARCHAR, -- 使用的技术栈
    start_date DATE,
    end_date DATE,
    url VARCHAR, -- 项目链接
    github_url VARCHAR, -- GitHub 链接
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 插入示例数据
INSERT INTO personal_info (name, title, email, phone, location, summary) VALUES 
('张三', '全栈开发工程师', 'zhangsan@example.com', '+86 138-0000-0000', '北京市', 
'具有5年以上全栈开发经验的软件工程师，专注于 Rust、JavaScript 和现代 Web 技术。热爱开源项目，具备良好的团队协作能力和技术领导经验。');

INSERT INTO skills (name, category, proficiency) VALUES 
('Rust', '编程语言', 5),
('JavaScript', '编程语言', 4),
('Python', '编程语言', 4),
('React', '前端框架', 4),
('Vue.js', '前端框架', 3),
('Rocket', 'Web框架', 4),
('Diesel', 'ORM', 4),
('PostgreSQL', '数据库', 4),
('SQLite', '数据库', 5),
('Git', '版本控制', 5),
('Docker', '容器化', 3),
('Linux', '操作系统', 4);

INSERT INTO work_experiences (company, position, start_date, end_date, description, achievements) VALUES 
('科技创新有限公司', '高级全栈开发工程师', '2022-01-01', NULL, 
'负责公司核心产品的全栈开发，使用 Rust 和 React 技术栈构建高性能 Web 应用。',
'• 主导重构了核心 API 服务，性能提升 300%\n• 建立了完整的 CI/CD 流程\n• 指导 3 名初级开发人员'),
('互联网科技公司', '全栈开发工程师', '2020-03-01', '2021-12-31', 
'参与多个 Web 应用的开发和维护，主要使用 JavaScript 和 Python 技术栈。',
'• 开发了用户管理系统，支持 10万+ 用户\n• 优化数据库查询，响应时间减少 50%\n• 参与代码审查和技术分享');

INSERT INTO education (institution, degree, field_of_study, start_date, end_date, gpa, description) VALUES 
('北京理工大学', '学士学位', '计算机科学与技术', '2016-09-01', '2020-06-30', '3.8/4.0', 
'主修计算机科学与技术，专注于软件工程和数据结构算法。参与了多个校内技术项目，获得优秀毕业生称号。');

INSERT INTO projects (name, description, technologies, start_date, end_date, url, github_url) VALUES 
('个人博客系统', '使用 Rust + Rocket + Diesel 开发的个人博客系统，支持 Markdown 编辑、标签分类、评论功能。', 
'Rust, Rocket, Diesel, SQLite, Tera, JavaScript', '2023-06-01', '2023-08-31', 
'https://blog.example.com', 'https://github.com/zhangsan/blog'),
('任务管理应用', '基于 React + Node.js 的团队任务管理应用，支持实时协作、文件上传、进度跟踪。', 
'React, Node.js, Express, MongoDB, Socket.io', '2022-09-01', '2022-12-31', 
'https://taskapp.example.com', 'https://github.com/zhangsan/taskapp'),
('数据可视化平台', '使用 Python + Django 开发的数据分析和可视化平台，支持多种图表类型和数据源。', 
'Python, Django, PostgreSQL, D3.js, Pandas', '2021-03-01', '2021-07-31', 
NULL, 'https://github.com/zhangsan/dataviz');

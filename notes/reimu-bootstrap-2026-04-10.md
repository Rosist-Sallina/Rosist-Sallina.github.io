# Reimu 站点初始化记录

时间：2026-04-10

## 目标

- 用 `hugo-theme-reimu` 重建 `Rosist-Sallina.github.io`
- 直接覆盖旧站内容
- 改成 GitHub Pages Actions 自动部署

## 已完成

- 清空旧站静态产物
- 复制 `hugo-reimu-template` 站点骨架
- 将主题以 submodule 方式接入到 `themes/hugo-theme-reimu`
- 主题版本固定到 `v0.15.4`
- 修改 `hugo.toml`
- 修改 `config/_default/params.yml`
- 新建初始文章 `content/post/hello-reimu.md`
- 改写 `about.md` 和 `friend.md`
- 新增 `.github/workflows/hugo.yml`
- 复用旧站 `assets/head.jpg` 作为头像
- 复用旧站 `assets/banner.jpg` 作为头图

## 当前待办

- 开始做页面魔改和旧文章迁移

## 最终状态

- 代码已推送到 `Rosist-Sallina/Rosist-Sallina.github.io`
- GitHub Pages 已切换到 `workflow` 发布模式
- `Deploy Hugo Site` 首次构建已成功
- 线上地址：`https://rosist-sallina.github.io/`

## 评论系统

- 已启用 GitHub Discussions
- 已接入 `Giscus`
- 使用仓库：`Rosist-Sallina/Rosist-Sallina.github.io`
- 使用分类：`General`
- 映射方式：`pathname`
- 由于上游主题当前把 `giscus.mapping` 错写成了 `strict`，已在站点侧通过 `layouts/partials/post/comment.html` 做本地覆盖修复
- 实测线上脚本已正确注入，但仓库尚未安装 `Giscus` GitHub App，因此评论框当前会报 `giscus is not installed on this repository`
- 安装入口：`https://github.com/apps/giscus/installations/new`

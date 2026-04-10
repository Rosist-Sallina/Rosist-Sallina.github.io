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

- 确认 GitHub Pages 的发布方式切到 `GitHub Actions`
- 等首次 Actions 构建完成
- 开始做页面魔改和旧文章迁移

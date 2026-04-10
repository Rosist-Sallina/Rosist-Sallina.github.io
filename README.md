# Rosist Sallina Blog

基于 [hugo-theme-reimu](https://github.com/D-Sketon/hugo-theme-reimu) 的个人博客。

## 本地开发

```bash
git clone --recursive https://github.com/Rosist-Sallina/Rosist-Sallina.github.io
cd Rosist-Sallina.github.io
hugo server
```

## 常改位置

- 站点配置：`hugo.toml`
- 主题配置：`config/_default/params.yml`
- 文章：`content/post`
- 关于页：`content/about.md`
- 友链页：`content/friend.md`
- 封面：`data/covers.yml`
- 头像：`static/avatar/avatar.jpg`
- 头图：`static/images/banner.jpg`

## 部署

- 推送到 `master` 后由 GitHub Actions 自动构建并发布到 GitHub Pages

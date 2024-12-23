(() => {
    const loadComments = async () => {
      if (typeof Gitalk === "undefined") {
        setTimeout(loadComments, 100);
      } else {
        const container = document.getElementById('gitalk-container');
        if (!container) {
          return;
        }
        const path = container.getAttribute("data-path");
        const gitalk = new Gitalk(Object.assign({
            id: path, // 直接使用路径作为 id
            // id: container.getAttribute("data-path-hash"), // 使用 hash 作为 ID
            path: path,
        }, {
          clientID: 'Ov23liOK4eLKQcAHcv04',
          clientSecret: '63193fd9eba6db7b44513576e57fea65c02811f5',
          repo: "Comments",
          owner: "Rosist-Sallina",
          admin: ["Rosist-Sallina"],
          distractionFreeMode: false,
        }));
  
        gitalk.render('gitalk-container');
      }
    };
  
    window.loadComments = loadComments;
    window.addEventListener('pjax:success', () => {
      window.loadComments = loadComments;
    });
  })();
  
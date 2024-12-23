(() => {
    const loadComments = async () => {
      if (typeof Gitment === "undefined") {
        setTimeout(loadComments, 100);
      } else {
        const container = document.getElementById('gitment-container');
        if (!container) {
          return;
        }
        const path = container.getAttribute("data-path");
        const gitment = new Gitment({
          id: path,
          owner: 'Rosist-Sallina',
          repo: 'Rosist-Sallina.github.io',
          oauth: {
            client_id: 'Ov23liOK4eLKQcAHcv04',
            client_secret: '63193fd9eba6db7b44513576e57fea65c02811f5',
          },
        });
  
        gitment.render('gitment-container');
      }
    };
  
    window.loadComments = loadComments;
    window.addEventListener('pjax:success', () => {
      window.loadComments = loadComments;
    });
  })();
  
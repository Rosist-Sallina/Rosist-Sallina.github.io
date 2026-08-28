/* 独立 CV 页交互：深浅色、滚动高亮、入场动画、打印 */
(function () {
  "use strict";

  var root = document.documentElement;

  /* ---- 深浅色切换 ---- */
  var toggle = document.getElementById("theme-toggle");
  if (toggle) {
    toggle.addEventListener("click", function () {
      var next = root.getAttribute("data-cv-theme") === "dark" ? "light" : "dark";
      root.setAttribute("data-cv-theme", next);
      try {
        localStorage.setItem("cv_theme", next);
      } catch (e) {}
    });
  }

  /* 未手动选择过时，跟随系统 */
  var mq = window.matchMedia("(prefers-color-scheme: dark)");
  var onScheme = function (e) {
    var saved = null;
    try {
      saved = localStorage.getItem("cv_theme");
    } catch (err) {}
    if (!saved) root.setAttribute("data-cv-theme", e.matches ? "dark" : "light");
  };
  if (mq.addEventListener) mq.addEventListener("change", onScheme);

  /* ---- 打印 ---- */
  var printBtn = document.getElementById("print-btn");
  if (printBtn) {
    printBtn.addEventListener("click", function () {
      window.print();
    });
  }

  /* ---- 入场动画 ---- */
  var items = document.querySelectorAll(".reveal");
  if (!("IntersectionObserver" in window)) {
    for (var i = 0; i < items.length; i++) items[i].classList.add("in");
  } else {
    var io = new IntersectionObserver(
      function (entries) {
        entries.forEach(function (en) {
          if (en.isIntersecting) {
            en.target.classList.add("in");
            io.unobserve(en.target);
          }
        });
      },
      { rootMargin: "0px 0px -40px 0px", threshold: 0.02 }
    );
    items.forEach(function (el) {
      io.observe(el);
    });
  }

  /* ---- 侧栏章节高亮 ---- */
  var links = [].slice.call(document.querySelectorAll(".rail-nav a"));
  var secs = links
    .map(function (a) {
      return document.querySelector(a.getAttribute("href"));
    })
    .filter(Boolean);

  if (secs.length) {
    var setOn = function (id) {
      links.forEach(function (a) {
        a.classList.toggle("on", a.getAttribute("href") === "#" + id);
      });
    };

    var spy = function () {
      // 固定 120px 判定线，避免视口高度影响结果
      var line = window.scrollY + 120;
      var cur = secs[0];
      for (var i = 0; i < secs.length; i++) {
        if (secs[i].offsetTop <= line) cur = secs[i];
      }
      // 滚到底部时锁定最后一节（仅当页面确实可滚动）
      var scrollable = document.documentElement.scrollHeight - window.innerHeight;
      if (scrollable > 40 && window.scrollY >= scrollable - 4) {
        cur = secs[secs.length - 1];
      }
      setOn(cur.id);
    };

    var tick = false;
    window.addEventListener(
      "scroll",
      function () {
        if (tick) return;
        tick = true;
        requestAnimationFrame(function () {
          spy();
          tick = false;
        });
      },
      { passive: true }
    );
    spy();
  }
})();

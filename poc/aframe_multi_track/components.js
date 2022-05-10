/* If you're feeling fancy you can add interactivity 
    to your site with Javascript */

AFRAME.registerComponent('navigate-on-click', {
  schema: {
    url: {
      default: '',
    },
  },
  init: function () {
    console.log('hello');
    console.log(this.el);
    var data = this.data;
    var el = this.el;
    el.addEventListener('click', function () {
      //window.location.href = data.url;
      window.open(data.url, '_blank');
    });
  },
});

AFRAME.registerComponent('color-randomizer', {
  init: function () {
    let colors = ['red', 'green', 'blue', 'black', 'orange', 'white'];
    var el = this.el;
    el.addEventListener('click', (e) => {
      this.el.setAttribute(
        'color',
        colors[Math.floor(Math.random() * colors.length)]
      );
    });
  },
});

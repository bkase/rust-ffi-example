var watch = require('watch');

task('default', [], function(params) {
  jake.Task['compile'].execute();
});

task('compile', [], function(params) {
  jake.exec('rustc *.rs', function() {
    complete();
  });
});

// TODO: Get this to work properly
task('watch', {async: true}, function(callback) {
  var rustRegex = /\.rs$/i;
  function handleModified(f) {
    if (f.match(rustRegex)) {
      jake.Task['compile'].execute();
    }
  }

  watch.createMonitor('.', function(monitor) {
    monitor.on('created', function(f, stat) {
      handleModified(f);
    });
    monitor.on('changed', function(f, stat) {
      handleModified(f);
    });
  });
});


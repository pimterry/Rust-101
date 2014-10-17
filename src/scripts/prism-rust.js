Prism.languages.rust = Prism.languages.extend('clike', {
	'keyword': /\b(fn|if|loop|struct|impl|actor|let|const|mut|enum)\b/g,
	'string': /@?("|')(\\?.)*?\1/g,
	'number': /\b-?(0x)?\d*\.?\d+.?\b/g
});

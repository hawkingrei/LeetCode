package main

import (
	"errors"
	"fmt"
	"math"
)

func divide(l, r int) (int, error) {
	m := (l + r) / 2
	if l < m && m < r {
		return m, nil
	} else {
		return 0, errors.New(fmt.Sprintf("[%d, %d) cannot be divided", l, r))
	}
}

type elem_t int

type merge_func_t func(int, int, int, elem_t, elem_t) elem_t

type node_t struct {
	elem        elem_t
	left, right *node_t
}

type segment_tree_t struct {
	root   *node_t
	merge  merge_func_t
	length int
}

func build(a []elem_t, merge merge_func_t) segment_tree_t {
	var build_iter func(l, r int) *node_t
	build_iter = func(l, r int) *node_t {
		m, err := divide(l, r)
		if err != nil {
			return &node_t{a[l], nil, nil}
		}
		node := node_t{}
		node.left, node.right = build_iter(l, m), build_iter(m, r)
		node.elem = merge(l, m, r, node.left.elem, node.right.elem)

		return &node
	}

	t := segment_tree_t{}
	t.root = build_iter(0, len(a))
	t.merge = merge
	t.length = len(a)

	return t
}

func (t *segment_tree_t) query(l, r int) (elem_t, error) {
	if r-l < 1 {
		var e elem_t
		return e, errors.New("Invalid query")
	}
	if l < 0 {
		l = 0
	}
	if r > t.length {
		r = t.length
	}

	var query_iter func(node *node_t, l, r, ql, qr int) elem_t
	query_iter = func(node *node_t, l, r, ql, qr int) elem_t {
		m, err := divide(l, r)
		if err != nil || (l == ql && r == qr) {
			return node.elem
		}
		if qr <= m {
			return query_iter(node.left, l, m, ql, qr)
		}
		if m <= ql {
			return query_iter(node.right, m, r, ql, qr)
		}
		e0 := query_iter(node.left, l, m, ql, m)
		e1 := query_iter(node.right, m, r, m, qr)

		return t.merge(ql, m, qr, e0, e1)
	}
	return query_iter(t.root, 0, t.length, l, r), nil
}

func (t *segment_tree_t) update(p int, e elem_t) {
	if p < 0 || p >= t.length {
		return
	}

	var update_iter func(node *node_t, l, r int)
	update_iter = func(node *node_t, l, r int) {
		m, err := divide(l, r)
		if err != nil {
			node.elem = e
			return
		}
		if p < m {
			update_iter(node.left, l, m)
		} else {
			update_iter(node.right, m, r)
		}
		node.elem = t.merge(l, m, r, node.left.elem, node.right.elem)
	}
	update_iter(t.root, 0, t.length)
}

func print_iter(node *node_t, l, r int) {
	if m, err := divide(l, r); err != nil {
		fmt.Println("    leaf", l, r, node.elem)
	} else {
		fmt.Println("internal", l, r, node.elem)
		print_iter(node.left, l, m)
		print_iter(node.right, m, r)
	}
}

func main() {
	a := []elem_t{1, 2, 3, 2, 3, 2, 1}

	st := build(a, func(l, m, r int, e0, e1 elem_t) elem_t {
		return e0 + e1*elem_t(math.Pow10(m-l))
	})

	fmt.Println(st.query(1, 4))
	fmt.Println(st.query(3, 6))
	st.update(1, 1)
	st.update(2, 1)
	fmt.Println(st.query(1, 4))
	fmt.Println(st.query(3, 6))
	fmt.Println(st.query(5, 5))
	fmt.Println(st.query(5, 6))

	print_iter(st.root, 0, st.length)
}

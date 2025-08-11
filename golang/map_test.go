package main

import (
	"github.com/stretchr/testify/require"
	"testing"
)

// 遍历map的方式
func TestMapRead(t *testing.T) {
	m := make(map[string]string)
	m["k1"] = "v1"

	k, ok := m["k1"]
	require.True(t, ok)
	require.Equal(t, k, "v1")

	_, ok = m["k2"]
	require.False(t, ok)
}

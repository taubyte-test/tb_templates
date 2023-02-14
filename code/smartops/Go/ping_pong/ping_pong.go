package lib

import (
	"github.com/taubyte/go-sdk-smartops/resource"
)

//export confirmHttp
func confirmHttp(r resource.Resource) uint32 {
	_, err := r.Function().Http()
	if err != nil {
		return 1
	}

	return 0
}

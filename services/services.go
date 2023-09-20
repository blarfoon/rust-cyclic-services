package services

type Services struct {
	Service1   Service1
	Service2   Service2
	SomeString string
	SomeStruct struct {
		SomeString string
	}
}

func New() Services {
	services := Services{
		SomeString: "some string",
		SomeStruct: struct {
			SomeString string
		}{
			SomeString: "some string",
		},
	}

	services.Service1.services = &services
	services.Service2.services = &services
	return services
}

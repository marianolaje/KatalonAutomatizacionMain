import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper as JsonSlurper

JsonSlurper slurper = new JsonSlurper()

println(t_response.responseBodyContent)

Map parsedJson = slurper.parseText(t_response.responseBodyContent)

'si retorno datos'
if ((parsedJson.data != null) && (parsedJson.data.inmueble != null)) {
	def array1 = parsedJson.data.inmueble

	def bandera = false

	for (def terreno : array1) {
		if (terreno.t_campo == t_varCampo) {
			bandera = true

			break
		}
	}
	
	if (!(bandera)) {
		

		KeywordUtil.markFailed(('Existe deuda del inmueble para el cuit ') + t_cuit)
	}
} else {
	println(v_dominio)

	println(t_cuit)

	KeywordUtil.markFailed(('no se encontro el cuit ') + t_cuit)
}

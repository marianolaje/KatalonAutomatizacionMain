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
import com.kms.katalon.core.logging.KeywordLogger


JsonSlurper slurper = new JsonSlurper()

println(t_response.responseBodyContent)

Map parsedJson = slurper.parseText(t_response.responseBodyContent)
def objeto = parsedJson.data.objetos.obligaciones
KeywordLogger log = new KeywordLogger()

'si retorno datos'
if ((objeto != null) && (objeto.idObligacion != null) && (objeto.saldoTotal != null) && (objeto.bloqueoEmision != null) && (objeto.descripcionEstado != null)) {
	
	def bloq = objeto.bloqueoEmision
	def desc = objeto.descripcionEstado
	def arrayBloq = bloq[0][0]
	def arrayDesc = desc[0][0]
	
	def bandera = false
	def banderaDos = false
	
	for (def bloqueo : arrayBloq) {
		log.logInfo('DATA INFO BloqueoEmision ------------> : ' + bloqueo)
		if (bloqueo.equals('S')) {
			bandera = true
			break
		}
	}
	
	for (def descripcion : arrayDesc) {
		log.logInfo('DATA INFO DescripcionEstado ------------> : ' + descripcion)
		if (descripcion.equals('Anulado') || descripcion.equals('Generado') || descripcion.equals('Anulada') || descripcion.equals('Generada')) {
			banderaDos = true
			break
		}
	}
	
	if (bandera) {
		
		KeywordUtil.markFailed(('Error en BloqueoEmision para el cuit ') + t_cuit)
		
	} 
	if(banderaDos){
		
		KeywordUtil.markFailed(('Error en DescripcionEstado para el cuit ') + t_cuit)
		
	} 
} else {

	println(t_cuit)

	KeywordUtil.markFailed(('no se encontro el cuit ') + t_cuit)
}




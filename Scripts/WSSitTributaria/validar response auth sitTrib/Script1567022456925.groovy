import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.logging.KeywordLogger
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.util.KeywordUtil
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
if (parsedJson.data != null) {
	
	def sit = parsedJson.data.situacionTributaria
	def texto = 'REGULARIZADA'
	
	KeywordLogger log = new KeywordLogger()
	log.logInfo('--------------------------->' + sit)
	
		if (sit == texto) {
			KeywordUtil.markPassed(('passed ') + t_cuit)
		} else {
		
			KeywordUtil.markFailed(('failed ') + t_cuit)
		}
} else {
	KeywordUtil.markFailed(('failed ') + t_cuit)
}
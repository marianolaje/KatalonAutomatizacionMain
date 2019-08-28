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

WebUI.callTestCase(findTestCase('oauth-server/login b2b ok'), [('t_servidor') : GlobalVariable.g_servidor_oauth, ('t_http_username') : 'klk-client'
        , ('t_http_password') : 'klk-secret', ('t_username') : t_username, ('t_password') : t_password], FailureHandling.STOP_ON_FAILURE)

response = WS.sendRequest(findTestObject('WSInmuebles/c2 mi inmueble (CUENTA)', [('r_access_token') : GlobalVariable.g_access_token
            , ('r_cuenta') : t_cuenta, ('r_servidor') : GlobalVariable.g_servidor_crm]))

WS.verifyResponseStatusCode(response, 200)

WebUI.callTestCase(findTestCase('WSInmuebles/WSInmueblesCUENTA/validar response mi inmueble (CUENTA)'), [('t_response') : response, ('t_cuenta') : t_cuenta
			, ('t_estado') : v_estado], FailureHandling.STOP_ON_FAILURE)

